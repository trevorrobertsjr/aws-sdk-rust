// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_create_ledger_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLedgerInput,
) {
    if let Some(var_1) = &input.deletion_protection {
        object.key("DeletionProtection").boolean(*var_1);
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2);
    }
    if let Some(var_3) = &input.permissions_mode {
        object.key("PermissionsMode").string(var_3.as_str());
    }
    if let Some(var_4) = &input.tags {
        let mut object_5 = object.key("Tags").start_object();
        for (key_6, value_7) in var_4 {
            if let Some(var_8) = value_7 {
                object_5.key(key_6).string(var_8);
            } else {
                object_5.key(key_6).null();
            }
        }
        object_5.finish();
    }
}

pub fn serialize_structure_export_journal_to_s3_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ExportJournalToS3Input,
) {
    if let Some(var_9) = &input.exclusive_end_time {
        object
            .key("ExclusiveEndTime")
            .instant(var_9, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_10) = &input.inclusive_start_time {
        object
            .key("InclusiveStartTime")
            .instant(var_10, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_11) = &input.role_arn {
        object.key("RoleArn").string(var_11);
    }
    if let Some(var_12) = &input.s3_export_configuration {
        let mut object_13 = object.key("S3ExportConfiguration").start_object();
        crate::json_ser::serialize_structure_s3_export_configuration(&mut object_13, var_12);
        object_13.finish();
    }
}

pub fn serialize_structure_get_block_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetBlockInput,
) {
    if let Some(var_14) = &input.block_address {
        let mut object_15 = object.key("BlockAddress").start_object();
        crate::json_ser::serialize_structure_value_holder(&mut object_15, var_14);
        object_15.finish();
    }
    if let Some(var_16) = &input.digest_tip_address {
        let mut object_17 = object.key("DigestTipAddress").start_object();
        crate::json_ser::serialize_structure_value_holder(&mut object_17, var_16);
        object_17.finish();
    }
}

pub fn serialize_structure_get_revision_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetRevisionInput,
) {
    if let Some(var_18) = &input.block_address {
        let mut object_19 = object.key("BlockAddress").start_object();
        crate::json_ser::serialize_structure_value_holder(&mut object_19, var_18);
        object_19.finish();
    }
    if let Some(var_20) = &input.digest_tip_address {
        let mut object_21 = object.key("DigestTipAddress").start_object();
        crate::json_ser::serialize_structure_value_holder(&mut object_21, var_20);
        object_21.finish();
    }
    if let Some(var_22) = &input.document_id {
        object.key("DocumentId").string(var_22);
    }
}

pub fn serialize_structure_stream_journal_to_kinesis_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StreamJournalToKinesisInput,
) {
    if let Some(var_23) = &input.exclusive_end_time {
        object
            .key("ExclusiveEndTime")
            .instant(var_23, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_24) = &input.inclusive_start_time {
        object
            .key("InclusiveStartTime")
            .instant(var_24, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_25) = &input.kinesis_configuration {
        let mut object_26 = object.key("KinesisConfiguration").start_object();
        crate::json_ser::serialize_structure_kinesis_configuration(&mut object_26, var_25);
        object_26.finish();
    }
    if let Some(var_27) = &input.role_arn {
        object.key("RoleArn").string(var_27);
    }
    if let Some(var_28) = &input.stream_name {
        object.key("StreamName").string(var_28);
    }
    if let Some(var_29) = &input.tags {
        let mut object_30 = object.key("Tags").start_object();
        for (key_31, value_32) in var_29 {
            if let Some(var_33) = value_32 {
                object_30.key(key_31).string(var_33);
            } else {
                object_30.key(key_31).null();
            }
        }
        object_30.finish();
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_34) = &input.tags {
        let mut object_35 = object.key("Tags").start_object();
        for (key_36, value_37) in var_34 {
            if let Some(var_38) = value_37 {
                object_35.key(key_36).string(var_38);
            } else {
                object_35.key(key_36).null();
            }
        }
        object_35.finish();
    }
}

pub fn serialize_structure_update_ledger_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLedgerInput,
) {
    if let Some(var_39) = &input.deletion_protection {
        object.key("DeletionProtection").boolean(*var_39);
    }
}

pub fn serialize_structure_update_ledger_permissions_mode_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLedgerPermissionsModeInput,
) {
    if let Some(var_40) = &input.permissions_mode {
        object.key("PermissionsMode").string(var_40.as_str());
    }
}

pub fn serialize_structure_s3_export_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3ExportConfiguration,
) {
    if let Some(var_41) = &input.bucket {
        object.key("Bucket").string(var_41);
    }
    if let Some(var_42) = &input.prefix {
        object.key("Prefix").string(var_42);
    }
    if let Some(var_43) = &input.encryption_configuration {
        let mut object_44 = object.key("EncryptionConfiguration").start_object();
        crate::json_ser::serialize_structure_s3_encryption_configuration(&mut object_44, var_43);
        object_44.finish();
    }
}

pub fn serialize_structure_value_holder(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ValueHolder,
) {
    if let Some(var_45) = &input.ion_text {
        object.key("IonText").string(var_45);
    }
}

pub fn serialize_structure_kinesis_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::KinesisConfiguration,
) {
    if let Some(var_46) = &input.stream_arn {
        object.key("StreamArn").string(var_46);
    }
    if let Some(var_47) = &input.aggregation_enabled {
        object.key("AggregationEnabled").boolean(*var_47);
    }
}

pub fn serialize_structure_s3_encryption_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3EncryptionConfiguration,
) {
    if let Some(var_48) = &input.object_encryption_type {
        object.key("ObjectEncryptionType").string(var_48.as_str());
    }
    if let Some(var_49) = &input.kms_key_arn {
        object.key("KmsKeyArn").string(var_49);
    }
}
