use std::sync::mpsc::Receiver;

use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 0;

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
		assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

		System::assert_last_event(Event::KittyCreated{
			who: account_id,
			kitty_id,
			kitty: KittiesModule::kitties(kitty_id).unwrap_or_default(),
		}.into());

		crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
		assert_noop!(
			KittiesModule::create(RuntimeOrigin::signed(account_id)),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn breed() {
		new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id),
			Error::<Test>::SameKittyId
		);

		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, 2),
			Error::<Test>::InvalidKittyId
		);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

		let cur_kitty_id = 2;
		assert_eq!(KittiesModule::next_kitty_id(), cur_kitty_id);
		assert_ok!(
			KittiesModule::breed(
				RuntimeOrigin::signed(account_id),
				kitty_id,
				kitty_id + 1,
			),
		);

		assert_eq!(KittiesModule::next_kitty_id(), cur_kitty_id + 1);
		assert_eq!(KittiesModule::kitties(cur_kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(cur_kitty_id), Some(account_id));
		assert_eq!(
			KittiesModule::kitty_parents(cur_kitty_id), Some((kitty_id, kitty_id + 1))
		);

		System::assert_last_event(Event::KittyBred {
			who: account_id,
			kitty_id: cur_kitty_id,
			kitty: KittiesModule::kitties(cur_kitty_id).unwrap_or_default(),
		}.into());
	});
}

#[test]
fn transfer() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let recipient = 2;

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

		assert_noop!(KittiesModule::transfer(
			RuntimeOrigin::signed(recipient),
			recipient,
			kitty_id,
		), Error::<Test>::NotOwner);

		// transfer to receipient
		assert_ok!(KittiesModule::transfer(
			RuntimeOrigin::signed(account_id),
			recipient,
			kitty_id,
		));

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recipient));

		System::assert_last_event(Event::KittyTransferred {
			who: account_id,
			to: recipient,
			kitty_id: kitty_id,
		}.into());

		// transfer back
		assert_ok!(KittiesModule::transfer(
			RuntimeOrigin::signed(recipient),
			account_id,
			kitty_id,
		));

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

		System::assert_last_event(Event::KittyTransferred {
			who: recipient,
			to: account_id,
			kitty_id: kitty_id,
		}.into());
	});
}