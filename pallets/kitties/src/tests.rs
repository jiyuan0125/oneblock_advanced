use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_create() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
		assert!(KittiesModule::kitties(kitty_id).is_some());
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert!(KittiesModule::kitty_parents(kitty_id).is_none());

		let kitty = KittiesModule::kitties(kitty_id).unwrap();

		// Assert that the correct event was deposited
		System::assert_last_event(
			Event::KittyCreated { who: account_id, kitty_id: 0, kitty }.into(),
		);

		crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
		assert_noop!(
			KittiesModule::create(RuntimeOrigin::signed(account_id)),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn it_works_for_breed() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id),
			Error::<Test>::SamedKittyId
		);

		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1),
			Error::<Test>::InvalidKittyId
		);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

		assert_ok!(KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1));

		let breed_kitty_id = 2;
		assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
		assert!(KittiesModule::kitties(breed_kitty_id).is_some());
		assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(breed_kitty_id), Some((kitty_id, kitty_id + 1)));

		let breed_kitty = KittiesModule::kitties(breed_kitty_id).unwrap();
		System::assert_last_event(Event::KittyBred { who: account_id, kitty_id: 2, kitty: breed_kitty }.into());
	});
}

#[test]
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let recipient = 2;

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

		assert_noop!(
			KittiesModule::transfer(RuntimeOrigin::signed(recipient), recipient, kitty_id),
			Error::<Test>::NotOwner
		);

		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id), recipient, kitty_id));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recipient));
		System::assert_last_event(
			Event::KittyTransferred { who: account_id, recipient, kitty_id: 0 }.into(),
		);

		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(recipient), recipient, kitty_id));
		System::assert_last_event(
			Event::KittyTransferred { who: recipient, recipient, kitty_id: 0 }.into(),
		);

	});
}
