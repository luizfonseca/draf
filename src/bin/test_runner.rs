//! Test runner for the Draf TypeScript compiler
//!
//! This script automatically runs all test files in the tests/ directory
//! and reports success or failure for each one.

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

/// ANSI color codes for terminal output
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

/// Test result status
#[derive(Debug, PartialEq, Clone)]
enum TestResult {
    Success,
    CompilationError(String),
    NotFound,
}

/// Individual test case information
#[derive(Clone)]
struct TestCase {
    name: String,
    path: String,
    result: TestResult,
    duration_ms: u64,
}

impl TestCase {
    fn new(name: String, path: String) -> Self {
        Self {
            name,
            path,
            result: TestResult::NotFound,
            duration_ms: 0,
        }
    }
}

/// Test runner configuration
struct TestRunner {
    project_root: String,
    verbose: bool,
    stop_on_first_failure: bool,
    benchmark: bool,
}

impl TestRunner {
    fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let verbose = args.contains(&"--verbose".to_string()) || args.contains(&"-v".to_string());
        let stop_on_first_failure =
            args.contains(&"--fail-fast".to_string()) || args.contains(&"-f".to_string());
        let benchmark =
            args.contains(&"--benchmark".to_string()) || args.contains(&"-b".to_string());

        Self {
            project_root: env::current_dir()
                .expect("Failed to get current directory")
                .to_string_lossy()
                .to_string(),
            verbose,
            stop_on_first_failure,
            benchmark,
        }
    }

    /// Discover all TypeScript test files
    fn discover_tests(&self) -> Vec<TestCase> {
        let tests_dir = Path::new(&self.project_root).join("tests");
        let mut test_cases = Vec::new();

        if !tests_dir.exists() {
            eprintln!("{}Error: tests/ directory not found{}", RED, RESET);
            return test_cases;
        }

        match fs::read_dir(&tests_dir) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if let Some(extension) = path.extension() {
                            if extension == "ts" {
                                if let Some(file_name) = path.file_stem() {
                                    let name = file_name.to_string_lossy().to_string();
                                    let full_path = path.to_string_lossy().to_string();
                                    test_cases.push(TestCase::new(name, full_path));
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("{}Error reading tests directory: {}{}", RED, e, RESET);
                return test_cases;
            }
        }

        // Sort test cases by name for consistent output
        test_cases.sort_by(|a, b| a.name.cmp(&b.name));
        test_cases
    }

    /// Run a single test case
    fn run_test(&self, test_case: &mut TestCase) {
        let start_time = Instant::now();

        // Build the draf compiler command
        let mut cmd = Command::new("cargo");
        cmd.arg("run")
            .arg("--bin")
            .arg("draf")
            .arg("--quiet")
            .arg("--")
            .arg("-o")
            .arg(format!("bins/{}", test_case.name))
            .arg(&test_case.path);

        if self.verbose {
            println!(
                "{}Running: cargo run --bin draf -- {}{}",
                BLUE, test_case.path, RESET
            );
        }

        // Execute the command
        match cmd.output() {
            Ok(output) => {
                let duration = start_time.elapsed();
                test_case.duration_ms = duration.as_millis() as u64;

                if output.status.success() {
                    test_case.result = TestResult::Success;
                    if self.verbose {
                        println!(
                            "{}✓ {} ({}ms){}",
                            GREEN, test_case.name, test_case.duration_ms, RESET
                        );
                    }
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let mut error_msg = String::new();

                    if !stderr.is_empty() {
                        error_msg.push_str(&stderr);
                    }
                    if !stdout.is_empty() {
                        if !error_msg.is_empty() {
                            error_msg.push('\n');
                        }
                        error_msg.push_str(&stdout);
                    }

                    test_case.result = TestResult::CompilationError(error_msg.trim().to_string());

                    if self.verbose {
                        println!(
                            "{}✗ {} ({}ms){}",
                            RED, test_case.name, test_case.duration_ms, RESET
                        );
                        println!("  Error: {}", error_msg.trim());
                    }
                }
            }
            Err(e) => {
                test_case.result =
                    TestResult::CompilationError(format!("Failed to execute command: {}", e));
                if self.verbose {
                    println!("{}✗ {} (command failed){}", RED, test_case.name, RESET);
                }
            }
        }
    }

    /// Print test summary
    fn print_summary(&self, test_cases: &[TestCase]) {
        let total_tests = test_cases.len();
        let successful_tests = test_cases
            .iter()
            .filter(|t| t.result == TestResult::Success)
            .count();
        let failed_tests = total_tests - successful_tests;
        let total_duration: u64 = test_cases.iter().map(|t| t.duration_ms).sum();

        println!("\n{}=== Test Summary ==={}", BOLD, RESET);
        println!("Total tests: {}", total_tests);
        println!("{}Passed: {}{}", GREEN, successful_tests, RESET);

        if failed_tests > 0 {
            println!("{}Failed: {}{}", RED, failed_tests, RESET);
        }

        let execution_mode = if self.stop_on_first_failure {
            "sequential"
        } else {
            "parallel"
        };
        println!(
            "Total time: {}ms ({} execution)",
            total_duration, execution_mode
        );

        if failed_tests > 0 {
            println!("\n{}Failed Tests:{}", RED, RESET);
            for test_case in test_cases {
                if let TestResult::CompilationError(ref error) = test_case.result {
                    println!("  {}✗ {}{}", RED, test_case.name, RESET);
                    if self.verbose {
                        // Show first line of error for summary
                        let first_line = error.lines().next().unwrap_or("Unknown error");
                        println!("    {}", first_line);
                    }
                }
            }
        }

        println!(
            "\n{}Success Rate: {:.1}%{}",
            if failed_tests == 0 { GREEN } else { YELLOW },
            (successful_tests as f64 / total_tests as f64) * 100.0,
            RESET
        );
    }

    /// Print detailed test results
    fn print_detailed_results(&self, test_cases: &[TestCase]) {
        if !self.verbose {
            return;
        }

        println!("\n{}=== Detailed Results ==={}", BOLD, RESET);
        for test_case in test_cases {
            match &test_case.result {
                TestResult::Success => {
                    println!(
                        "{}✓{} {} ({}ms)",
                        GREEN, RESET, test_case.name, test_case.duration_ms
                    );
                }
                TestResult::CompilationError(error) => {
                    println!(
                        "{}✗{} {} ({}ms)",
                        RED, RESET, test_case.name, test_case.duration_ms
                    );
                    println!("  Error: {}", error);
                }
                TestResult::NotFound => {
                    println!("{}?{} {} (not run)", YELLOW, RESET, test_case.name);
                }
            }
        }
    }

    /// Run all tests
    fn run_all_tests(&self) {
        println!("{}Draf TypeScript Compiler Test Runner{}", BOLD, RESET);
        println!("Project root: {}", self.project_root);
        println!("Verbose mode: {}", if self.verbose { "ON" } else { "OFF" });
        println!(
            "Fail-fast mode: {}",
            if self.stop_on_first_failure {
                "ON"
            } else {
                "OFF"
            }
        );
        if self.benchmark {
            println!("Benchmark mode: ON");
        }
        println!();

        let mut test_cases = self.discover_tests();

        if test_cases.is_empty() {
            println!(
                "{}No TypeScript test files found in tests/ directory{}",
                YELLOW, RESET
            );
            return;
        }

        println!("Found {} test files", test_cases.len());
        if self.stop_on_first_failure {
            println!(
                "{}Running tests sequentially (fail-fast mode)...{}",
                BLUE, RESET
            );
        } else {
            println!("{}Running tests in parallel...{}", BLUE, RESET);
        }
        println!();

        let total_tests = test_cases.len();

        // Run benchmark if requested
        if self.benchmark {
            self.run_benchmark(&mut test_cases);
            return;
        }

        // Run tests in parallel or sequential based on configuration
        if self.stop_on_first_failure {
            // Sequential execution for fail-fast mode
            let mut progress = 0;
            for test_case in &mut test_cases {
                progress += 1;

                if !self.verbose {
                    print!("({}/{}) {} ... ", progress, total_tests, test_case.name);
                }

                self.run_test(test_case);

                if !self.verbose {
                    match &test_case.result {
                        TestResult::Success => {
                            println!("{}OK{} ({}ms)", GREEN, RESET, test_case.duration_ms)
                        }
                        TestResult::CompilationError(_) => {
                            println!("{}FAIL{} ({}ms)", RED, RESET, test_case.duration_ms)
                        }
                        TestResult::NotFound => println!("{}SKIP{}", YELLOW, RESET),
                    }
                }

                // Stop on first failure if requested
                if test_case.result != TestResult::Success {
                    println!(
                        "\n{}Stopping on first failure (--fail-fast mode){}",
                        YELLOW, RESET
                    );
                    break;
                }
            }
        } else {
            // Parallel execution
            self.run_tests_parallel(&mut test_cases);
        }

        self.print_summary(&test_cases);
        self.print_detailed_results(&test_cases);

        // Exit with error code if any tests failed
        let failed_count = test_cases
            .iter()
            .filter(|t| t.result != TestResult::Success)
            .count();
        if failed_count > 0 {
            std::process::exit(1);
        }
    }

    /// Run tests in parallel using a thread pool
    fn run_tests_parallel(&self, test_cases: &mut [TestCase]) {
        // Limit concurrency to avoid overwhelming the system
        let num_threads = std::cmp::min(
            std::cmp::max(1, num_cpus::get() / 2), // Use half the available cores
            std::cmp::min(8, test_cases.len()),    // Cap at 8 threads max
        );

        println!(
            "{}Using {} threads for parallel execution{}",
            BLUE, num_threads, RESET
        );

        // Create work queue and results storage
        let work_queue: Arc<Mutex<Vec<usize>>> =
            Arc::new(Mutex::new((0..test_cases.len()).collect()));
        let results: Arc<Mutex<Vec<Option<(TestResult, u64)>>>> =
            Arc::new(Mutex::new(vec![None; test_cases.len()]));
        let progress_counter = Arc::new(Mutex::new(0));
        let total_tests = test_cases.len();

        let mut handles = vec![];

        for _thread_id in 0..num_threads {
            let work_queue = Arc::clone(&work_queue);
            let results = Arc::clone(&results);
            let progress_counter = Arc::clone(&progress_counter);
            let project_root = self.project_root.clone();
            let verbose = self.verbose;

            // Clone test case data for this thread
            let test_data: Vec<(String, String)> = test_cases
                .iter()
                .map(|tc| (tc.name.clone(), tc.path.clone()))
                .collect();

            let handle = thread::spawn(move || {
                loop {
                    // Get next test index from work queue
                    let test_index = {
                        let mut queue = work_queue.lock().unwrap();
                        if queue.is_empty() {
                            break;
                        }
                        queue.pop().unwrap()
                    };

                    let (test_name, test_path) = &test_data[test_index];
                    let mut test_case = TestCase::new(test_name.clone(), test_path.clone());

                    // Create a local test runner for this thread
                    let runner = TestRunner {
                        project_root: project_root.clone(),
                        verbose: false, // Disable verbose for parallel to avoid output conflicts
                        stop_on_first_failure: false,
                        benchmark: false,
                    };

                    // Run the test
                    runner.run_test(&mut test_case);

                    // Update progress and print result (with thread-safe printing)
                    let current_progress = {
                        let mut progress = progress_counter.lock().unwrap();
                        *progress += 1;
                        *progress
                    };

                    if !verbose {
                        // Use a single print statement to avoid interleaving
                        let status_msg = match &test_case.result {
                            TestResult::Success => {
                                format!("{}OK{} ({}ms)", GREEN, RESET, test_case.duration_ms)
                            }
                            TestResult::CompilationError(_) => {
                                format!("{}FAIL{} ({}ms)", RED, RESET, test_case.duration_ms)
                            }
                            TestResult::NotFound => format!("{}SKIP{}", YELLOW, RESET),
                        };
                        println!(
                            "({}/{}) {} ... {}",
                            current_progress, total_tests, test_case.name, status_msg
                        );
                    }

                    // Store result
                    {
                        let mut result_vec = results.lock().unwrap();
                        result_vec[test_index] =
                            Some((test_case.result.clone(), test_case.duration_ms));
                    }
                }
            });

            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        // Copy results back to original test_cases
        let final_results = results.lock().unwrap();
        for (i, result_option) in final_results.iter().enumerate() {
            if let Some((result, duration)) = result_option {
                if i < test_cases.len() {
                    test_cases[i].result = result.clone();
                    test_cases[i].duration_ms = *duration;
                }
            }
        }
    }

    /// Run performance benchmark comparing sequential vs parallel execution
    fn run_benchmark(&self, test_cases: &mut [TestCase]) {
        println!("{}=== Performance Benchmark ==={}", BOLD, RESET);
        println!("Comparing sequential vs parallel test execution\n");

        // Run sequential benchmark
        println!("{}Running sequential benchmark...{}", BLUE, RESET);
        let start_time = Instant::now();
        let mut sequential_cases = test_cases.to_vec();
        let mut progress = 0;
        for test_case in &mut sequential_cases {
            progress += 1;
            print!(
                "({}/{}) {} ... ",
                progress,
                test_cases.len(),
                test_case.name
            );
            self.run_test(test_case);
            match &test_case.result {
                TestResult::Success => {
                    println!("{}OK{} ({}ms)", GREEN, RESET, test_case.duration_ms)
                }
                TestResult::CompilationError(_) => {
                    println!("{}FAIL{} ({}ms)", RED, RESET, test_case.duration_ms)
                }
                TestResult::NotFound => println!("{}SKIP{}", YELLOW, RESET),
            }
        }
        let sequential_time = start_time.elapsed();

        println!("\n{}Running parallel benchmark...{}", BLUE, RESET);
        let start_time = Instant::now();
        self.run_tests_parallel(test_cases);
        let parallel_time = start_time.elapsed();

        // Print benchmark results
        println!("\n{}=== Benchmark Results ==={}", BOLD, RESET);
        println!("Sequential execution: {}ms", sequential_time.as_millis());
        println!("Parallel execution:   {}ms", parallel_time.as_millis());

        let speedup = sequential_time.as_millis() as f64 / parallel_time.as_millis() as f64;
        println!("Speedup factor: {:.2}x", speedup);

        if speedup > 1.0 {
            println!(
                "{}Parallel execution is {:.1}% faster{}",
                GREEN,
                (speedup - 1.0) * 100.0,
                RESET
            );
        } else {
            println!(
                "{}Sequential execution is {:.1}% faster{}",
                YELLOW,
                (1.0 / speedup - 1.0) * 100.0,
                RESET
            );
        }
    }

    /// Print help message
    fn print_help() {
        println!("{}Draf TypeScript Compiler Test Runner{}", BOLD, RESET);
        println!();
        println!("Usage: cargo run --bin test_runner [OPTIONS]");
        println!();
        println!("Options:");
        println!("  -v, --verbose       Show detailed output for each test");
        println!(
            "  -f, --fail-fast     Stop on the first test failure (disables parallel execution)"
        );
        println!("  -b, --benchmark     Run performance benchmark (sequential vs parallel)");
        println!("  -h, --help          Show this help message");
        println!();
        println!("Note: Tests run in parallel by default for better performance.");
        println!("Use --fail-fast to run sequentially and stop on first failure.");
        println!();
        println!("Examples:");
        println!("  cargo run --bin test_runner");
        println!("  cargo run --bin test_runner --verbose");
        println!("  cargo run --bin test_runner --fail-fast");
        println!("  cargo run --bin test_runner --benchmark");
        println!("  cargo run --bin test_runner -v -f");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        TestRunner::print_help();
        return;
    }

    let runner = TestRunner::new();
    runner.run_all_tests();
}
