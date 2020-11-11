use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::add_person(Origin::signed(1), "name".as_bytes().to_vec(), 99));
	});
}
