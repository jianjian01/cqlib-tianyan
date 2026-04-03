/*
 * This code is part of Cqlib.
 *
 * (C) Copyright China Telecom Quantum Group 2026
 *
 * This code is licensed under the Apache License, Version 2.0.
 *
 * basic_usage.c — Demonstrates the full workflow for the Tianyan C API.
 *
 * Build (after `cargo build -p binding-c --release`):
 *
 *   cc -I../include basic_usage.c \
 *      -L../../../target/release \
 *      -lcqlib_tianyan_c \        # adjust to the actual .so/.dylib name
 *      -o basic_usage
 *
 * Or with a static library:
 *   cc -I../include basic_usage.c \
 *      ../../../target/release/libcqlib_tianyan_c.a \
 *      -o basic_usage
 *
 * Set the TIANYAN_API_KEY environment variable before running.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../include/cqlib_tianyan.h"

/* Print the last error and return 1 (for use in early-exit macros). */
static int print_last_error(const char* context) {
    char* err = tianyan_last_error();
    if (err) {
        fprintf(stderr, "[%s] Error: %s\n", context, err);
        tianyan_string_free(err);
    } else {
        fprintf(stderr, "[%s] Unknown error\n", context);
    }
    return 1;
}

int main(void) {
    /* ── 1. Login ─────────────────────────────────────────────────────────── */
    const char* api_key = getenv("TIANYAN_API_KEY");
    if (!api_key || strlen(api_key) == 0) {
        fprintf(stderr, "Set the TIANYAN_API_KEY environment variable first.\n");
        return 1;
    }

    printf("Logging in...\n");
    TianyanPlatformC* platform = tianyan_platform_login(api_key);
    if (!platform) {
        return print_last_error("login");
    }
    printf("Login successful.\n\n");

    /* ── 2. List backends ─────────────────────────────────────────────────── */
    size_t n_backends = 0;
    TianyanBackendC** backends = tianyan_platform_list_backends(platform, &n_backends);
    if (!backends) {
        tianyan_platform_free(platform);
        return print_last_error("list_backends");
    }

    printf("Available backends (%zu):\n", n_backends);
    const char* target_name = NULL;
    for (size_t i = 0; i < n_backends; i++) {
        int status = tianyan_backend_status(backends[i]);
        int toll = tianyan_backend_toll(backends[i]);
        printf("  [%zu] name=%-20s  status=%d  toll=%d  available=%s\n", i,
               tianyan_backend_name(backends[i]), status, toll,
               tianyan_backend_is_available(backends[i]) ? "yes" : "no");

        /* Pick the first available backend for the demo */
        if (!target_name && tianyan_backend_is_available(backends[i])) {
            target_name = tianyan_backend_name(backends[i]);
        }
    }
    printf("\n");

    if (!target_name) {
        fprintf(stderr, "No available backend found. Exiting.\n");
        tianyan_backend_list_free(backends, n_backends);
        tianyan_platform_free(platform);
        return 1;
    }

    /* ── 3. Get a backend handle ──────────────────────────────────────────── */
    TianyanBackendC* backend = tianyan_platform_get_backend(platform, target_name);
    if (!backend) {
        tianyan_backend_list_free(backends, n_backends);
        tianyan_platform_free(platform);
        return print_last_error("get_backend");
    }
    printf("Using backend: %s\n\n", tianyan_backend_name(backend));

    /* ── 4. Submit a Bell-state circuit ──────────────────────────────────── */
    /* H Q1, H Q8, CZ Q1 Q8, H Q8 ≡ H Q1 + CNOT(Q1→Q8) */
    const char* circuits[] = {"H Q1\nH Q8\nCZ Q1 Q8\nH Q8\nM Q1\nM Q8"};
    size_t n_circuits = 1;
    size_t shots = 1000;

    printf("Submitting %zu circuit(s) with %zu shots...\n", n_circuits, shots);
    TianyanTaskC* task = tianyan_backend_run(backend, circuits, n_circuits, shots);
    if (!task) {
        tianyan_backend_free(backend);
        tianyan_backend_list_free(backends, n_backends);
        tianyan_platform_free(platform);
        return print_last_error("backend_run");
    }

    /* Show the query IDs we received */
    size_t n_ids = 0;
    char** ids = tianyan_task_ids(task, &n_ids);
    printf("Submitted %zu circuit(s). Query IDs:\n", n_ids);
    for (size_t i = 0; i < n_ids; i++) {
        printf("  [%zu] %s\n", i, ids[i]);
    }
    tianyan_task_ids_free(ids, n_ids);
    printf("\n");

    /* ── 5. Wait for results ─────────────────────────────────────────────── */
    printf("Waiting for results (timeout=120s, poll=5s)...\n");
    TianyanResultList* results = tianyan_task_wait(task, 120.0, 5.0);
    if (!results) {
        tianyan_task_free(task);
        tianyan_backend_free(backend);
        tianyan_backend_list_free(backends, n_backends);
        tianyan_platform_free(platform);
        return print_last_error("task_wait");
    }

    /* ── 6. Print results ────────────────────────────────────────────────── */
    size_t n_results = tianyan_result_list_len(results);
    printf("Got %zu result(s):\n", n_results);
    for (size_t i = 0; i < n_results; i++) {
        const char* task_id = tianyan_result_task_id(results, i);
        size_t r_shots = tianyan_result_shots(results, i);
        size_t n_qubits = tianyan_result_num_qubits(results, i);
        char* counts = tianyan_result_counts_json(results, i);

        printf("  [%zu] task_id=%s  shots=%zu  num_qubits=%zu\n", i, task_id ? task_id : "(null)",
               r_shots, n_qubits);
        printf("       counts=%s\n", counts ? counts : "(null)");

        tianyan_string_free(counts);
    }

    /* ── 7. Clean up ─────────────────────────────────────────────────────── */
    tianyan_result_list_free(results);
    tianyan_task_free(task);
    tianyan_backend_free(backend);
    tianyan_backend_list_free(backends, n_backends);
    tianyan_platform_free(platform);

    printf("\nDone.\n");
    return 0;
}
