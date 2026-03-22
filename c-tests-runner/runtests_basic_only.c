/* Test runner - runs basic/ns/misc/acc tests, single pass, with crash recovery */
#include "expat_config.h"
#include <stdio.h>
#include <string.h>
#include <signal.h>
#include <setjmp.h>
#include <stdlib.h>
#include "expat.h"
#include "internal.h"
#include "minicheck.h"
#include "common.h"
#include "basic_tests.h"
#include "ns_tests.h"
#include "misc_tests.h"
#include "acc_tests.h"

XML_Parser g_parser = NULL;

static Suite *
make_suite(void) {
  Suite *s = suite_create("basic");
  make_basic_test_case(s);
  make_namespace_test_case(s);
  make_miscellaneous_test_case(s);
#if XML_GE == 1
  make_accounting_test_case(s);
#endif
  return s;
}

int
main(int argc, char *argv[]) {
  int nf;
  Suite *s = make_suite();
  SRunner *sr = srunner_create(s);

  printf("Expat version: %s\n", XML_ExpatVersion());
  fflush(stdout);

  g_chunkSize = 0;
  g_reparseDeferralEnabledDefault = XML_TRUE;
  srunner_run_all(sr, "basic", CK_VERBOSE);

  srunner_summarize(sr, CK_VERBOSE);
  nf = srunner_ntests_failed(sr);
  printf("\nTotal: %d, Passed: %d, Failed: %d\n",
         sr->nchecks, sr->nchecks - nf, nf);
  srunner_free(sr);
  return (nf == 0) ? 0 : 1;
}
