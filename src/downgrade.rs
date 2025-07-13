use kovi::event::{AdminMsgEvent, GroupMsgEvent};
use kovi::AllMsgEvent;
use std::sync::Arc;

pub fn downgrade_admin(msg: Arc<AdminMsgEvent>) -> Arc<AllMsgEvent> {
    Arc::new(AllMsgEvent {
        time: msg.time,
        self_id: msg.self_id,
        post_type: msg.post_type.clone(),
        message_type: msg.message_type.clone(),
        sub_type: msg.sub_type.clone(),
        message: msg.message.clone(),
        message_id: msg.message_id,
        group_id: msg.group_id,
        user_id: msg.user_id,
        anonymous: msg.anonymous.clone(),
        raw_message: msg.raw_message.clone(),
        font: msg.font,
        sender: msg.sender.clone(),
        text: msg.text.clone(),
        human_text: msg.human_text.clone(),
        original_json: msg.original_json.clone(),
        api_tx: msg.api_tx.clone(),
    })
}
pub fn downgrade_group(msg: Arc<GroupMsgEvent>) -> Arc<AllMsgEvent> {
    Arc::new(AllMsgEvent {
        time: msg.time,
        self_id: msg.self_id,
        post_type: msg.post_type.clone(),
        message_type: msg.message_type.clone(),
        sub_type: msg.sub_type.clone(),
        message: msg.message.clone(),
        message_id: msg.message_id,
        group_id: Some(msg.group_id),
        user_id: msg.user_id,
        anonymous: msg.anonymous.clone(),
        raw_message: msg.raw_message.clone(),
        font: msg.font,
        sender: msg.sender.clone(),
        text: msg.text.clone(),
        human_text: msg.human_text.clone(),
        original_json: msg.original_json.clone(),
        api_tx: msg.api_tx.clone(),
    })
}
