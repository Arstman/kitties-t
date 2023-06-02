use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;

#[test]
fn create_kitty_works() {
	new_test_ext().execute_with(|| {
		let account_id = 1;
		let kitty_id = 0;

		assert_eq!(Kitties::next_kitty_id(), kitty_id);
		assert_ok!(Kitties::create(RuntimeOrigin::signed(account_id)));

		assert_eq!(Kitties::next_kitty_id(), kitty_id + 1);
		assert_eq!(Kitties::kitties(kitty_id).is_some(), true);
		assert_eq!(Kitties::kitty_owner(kitty_id), Some(account_id));
		assert_eq!(Kitties::kitty_parents(kitty_id), None);

		let kitty = Kitties::kitties(kitty_id).expect(" Kitty Created ");
		System::assert_last_event(Event::KittyCreated { who: account_id, kitty_id, kitty }.into());
	});
}

#[test]
fn create_kitty_failed_when_bad_origin() {
	new_test_ext().execute_with(|| {
		assert_noop!(Kitties::create(RuntimeOrigin::root()), BadOrigin);
		assert_noop!(Kitties::create(RuntimeOrigin::none()), BadOrigin);
	});
}

#[test]
fn create_kitty_failed_when_next_kitty_id_overflow() {
	new_test_ext().execute_with(|| {
		let creator = 1;

		crate::NextKittyId::<Test>::set(crate::KittyId::MAX);
		assert_noop!(
			Kitties::create(RuntimeOrigin::signed(creator)),
			Error::<Test>::KittyIdCannotOverflow
		);
	});
}



#[test]
fn breed_kitty_works() {
	new_test_ext().execute_with(|| {
		let creator = 1;
		let kitty_id_1 = 0;
		let kitty_id_2 = kitty_id_1 + 1;
		let bred_kitty_id = kitty_id_1 + 2;

		assert_eq!(Kitties::next_kitty_id(), kitty_id_1);
		assert_ok!(Kitties::create(RuntimeOrigin::signed(creator)));

		assert_eq!(Kitties::next_kitty_id(), kitty_id_2);
		assert_ok!(Kitties::create(RuntimeOrigin::signed(creator)));

		assert_eq!(Kitties::next_kitty_id(), bred_kitty_id);
		assert_ok!(Kitties::breed(RuntimeOrigin::signed(creator), kitty_id_1, kitty_id_2));

		assert!(Kitties::kitties(bred_kitty_id).is_some());
		assert_eq!(Kitties::kitty_owner(bred_kitty_id), Some(creator));
		assert_eq!(Kitties::kitty_parents(bred_kitty_id), Some((kitty_id_1, kitty_id_2)));

		let bred_kitty = Kitties::kitties(bred_kitty_id).expect("Kitty Breeded");
		System::assert_last_event(
			Event::KittyBred { who: creator, kitty_id: bred_kitty_id, kitty: bred_kitty }.into(),
		);
	});
}

#[test]
fn breed_kitty_failed_when_bad_origin() {
	new_test_ext().execute_with(|| {
		let creator = 1;

		let kitty_id = 0;
		assert_eq!(Kitties::next_kitty_id(), kitty_id);
		assert_ok!(Kitties::create(RuntimeOrigin::signed(creator)));
		assert_ok!(Kitties::create(RuntimeOrigin::signed(creator)));

		let bred_kitty_id = kitty_id + 2;
		assert_eq!(Kitties::next_kitty_id(), bred_kitty_id);

		assert_noop!(Kitties::breed(RuntimeOrigin::none(), kitty_id, kitty_id + 1), BadOrigin);
		assert_noop!(Kitties::breed(RuntimeOrigin::root(), kitty_id, kitty_id + 1), BadOrigin);
	});
}

#[test]
fn breed_kitty_failed_when_parents_using_the_same_kitty_id() {
	new_test_ext().execute_with(|| {
		let creator = 1;

		let kitty_id = 0;
		assert_eq!(Kitties::next_kitty_id(), kitty_id);
		assert_ok!(Kitties::create(RuntimeOrigin::signed(creator)));
		assert_ok!(Kitties::create(RuntimeOrigin::signed(creator)));

		let bred_kitty_id = kitty_id + 2;
		assert_eq!(Kitties::next_kitty_id(), bred_kitty_id);

		assert_noop!(
			Kitties::breed(RuntimeOrigin::signed(creator), kitty_id, kitty_id),
			Error::<Test>::SameKittyId
		);
	});
}

#[test]
fn transfer_kitty_works() {
	new_test_ext().execute_with(|| {
		let account_1 = 1;
		let account_2 = 2;

		let kitty_id = 0;
		assert_eq!(Kitties::next_kitty_id(), kitty_id);
		assert_ok!(Kitties::create(RuntimeOrigin::signed(account_1)));
		assert_eq!(Kitties::kitty_owner(kitty_id), Some(account_1));

		assert_ok!(Kitties::transfer(RuntimeOrigin::signed(account_1), account_2, kitty_id));
		assert_eq!(Kitties::kitty_owner(kitty_id), Some(account_2));
		System::assert_last_event(
			Event::KittyTransferred { who: account_1, recipient: account_2, kitty_id }.into(),
		);

		assert_ok!(Kitties::transfer(RuntimeOrigin::signed(account_2), account_1, kitty_id));
		assert_eq!(Kitties::kitty_owner(kitty_id), Some(account_1));
		System::assert_last_event(
			Event::KittyTransferred { who: account_2, recipient: account_1, kitty_id }.into(),
		);
	});
}

#[test]
fn transfer_kitty_failed_when_bad_origin() {
	new_test_ext().execute_with(|| {
		let account_1 = 1;
		let account_2 = 2;

		let kitty_id = 0;
		assert_eq!(Kitties::next_kitty_id(), kitty_id);
		assert_ok!(Kitties::create(RuntimeOrigin::signed(account_1)));
		assert_eq!(Kitties::kitty_owner(kitty_id), Some(account_1));

		assert_noop!(Kitties::transfer(RuntimeOrigin::root(), account_2, kitty_id), BadOrigin);
	});
}

#[test]
fn transfer_kitty_failed_when_invalid_kitty_id() {
	new_test_ext().execute_with(|| {
		let account_1 = 1;
		let account_2 = 2;

		let kitty_id = 0;
		let invalid_kitty_id = 100;

		assert_eq!(Kitties::next_kitty_id(), kitty_id);
		assert_ok!(Kitties::create(RuntimeOrigin::signed(account_1)));
		assert_eq!(Kitties::kitty_owner(kitty_id), Some(account_1));

		assert_noop!(
			Kitties::transfer(RuntimeOrigin::signed(account_1), account_2, invalid_kitty_id),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn transfer_kitty_failed_when_sender_was_not_kitty_owner() {
	new_test_ext().execute_with(|| {
		let account_1 = 1;
		let account_2 = 2;

		let kitty_id = 0;
		assert_eq!(Kitties::next_kitty_id(), kitty_id);
		assert_ok!(Kitties::create(RuntimeOrigin::signed(account_1)));
		assert_eq!(Kitties::kitty_owner(kitty_id), Some(account_1));

		assert_noop!(
			Kitties::transfer(RuntimeOrigin::signed(account_2), account_1, kitty_id),
			Error::<Test>::NotOwner
		);
	});
}
