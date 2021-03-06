#[test]
pub fn test_expansions()
{
	use crate::utils::ExpansionTester;
	let mut test = ExpansionTester::new("tests/module_disambiguation", "testing");
	test.add_source_dir("from", ExpansionTester::copy());
	test.add_source_dir("expected", ExpansionTester::copy());
	test.add_source_dir("expected_both", ExpansionTester::duplicate_for_syntaxes());
	test.execute_tests();
}
