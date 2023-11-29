#include "EZtest"
#include <stdio.h>

constexpr const int ____SIMPLETEST_MAX_TESTS = 1000;

typedef struct Test {
    const char* name;
    ____EZTEST_BOOL(*func)();
} Test;

Test tests[____SIMPLETEST_MAX_TESTS];

____EZTEST_BOOL ____EZTEST_register_test(const char* name, ____EZTEST_BOOL(*func)()) {
    for (int i = 0; i < ____SIMPLETEST_MAX_TESTS; i++) {
        if (tests[i].name == NULL) {
            tests[i].name = name;
            tests[i].func = func;
            return ____EZTEST_TRUE;
        }
    }
    return ____EZTEST_FALSE;
}

int ____EZTEST_run_tests() {
    int failed = 0;
    for (int i = 0; i < ____SIMPLETEST_MAX_TESTS; i++) {
        if (tests[i].name == NULL) break;
        printf("Running test %s...\n", tests[i].name);
        if (!tests[i].func()) {
            printf("Test %s failed!\n", tests[i].name);
            failed++;
        } else {
            printf("Test %s passed!\n", tests[i].name);
        }
    }
    return failed;
}
