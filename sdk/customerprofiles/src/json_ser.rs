// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_add_profile_key_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddProfileKeyInput,
) {
    if let Some(var_1) = &input.key_name {
        object.key("KeyName").string(var_1);
    }
    if let Some(var_2) = &input.profile_id {
        object.key("ProfileId").string(var_2);
    }
    if let Some(var_3) = &input.values {
        let mut array_4 = object.key("Values").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5);
            }
        }
        array_4.finish();
    }
}

pub fn serialize_structure_create_domain_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDomainInput,
) {
    if let Some(var_6) = &input.dead_letter_queue_url {
        object.key("DeadLetterQueueUrl").string(var_6);
    }
    if let Some(var_7) = &input.default_encryption_key {
        object.key("DefaultEncryptionKey").string(var_7);
    }
    if let Some(var_8) = &input.default_expiration_days {
        object.key("DefaultExpirationDays").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    if let Some(var_9) = &input.matching {
        let mut object_10 = object.key("Matching").start_object();
        crate::json_ser::serialize_structure_matching_request(&mut object_10, var_9);
        object_10.finish();
    }
    if let Some(var_11) = &input.tags {
        let mut object_12 = object.key("Tags").start_object();
        for (key_13, value_14) in var_11 {
            {
                object_12.key(key_13).string(value_14);
            }
        }
        object_12.finish();
    }
}

pub fn serialize_structure_create_profile_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateProfileInput,
) {
    if let Some(var_15) = &input.account_number {
        object.key("AccountNumber").string(var_15);
    }
    if let Some(var_16) = &input.additional_information {
        object.key("AdditionalInformation").string(var_16);
    }
    if let Some(var_17) = &input.address {
        let mut object_18 = object.key("Address").start_object();
        crate::json_ser::serialize_structure_address(&mut object_18, var_17);
        object_18.finish();
    }
    if let Some(var_19) = &input.attributes {
        let mut object_20 = object.key("Attributes").start_object();
        for (key_21, value_22) in var_19 {
            {
                object_20.key(key_21).string(value_22);
            }
        }
        object_20.finish();
    }
    if let Some(var_23) = &input.billing_address {
        let mut object_24 = object.key("BillingAddress").start_object();
        crate::json_ser::serialize_structure_address(&mut object_24, var_23);
        object_24.finish();
    }
    if let Some(var_25) = &input.birth_date {
        object.key("BirthDate").string(var_25);
    }
    if let Some(var_26) = &input.business_email_address {
        object.key("BusinessEmailAddress").string(var_26);
    }
    if let Some(var_27) = &input.business_name {
        object.key("BusinessName").string(var_27);
    }
    if let Some(var_28) = &input.business_phone_number {
        object.key("BusinessPhoneNumber").string(var_28);
    }
    if let Some(var_29) = &input.email_address {
        object.key("EmailAddress").string(var_29);
    }
    if let Some(var_30) = &input.first_name {
        object.key("FirstName").string(var_30);
    }
    if let Some(var_31) = &input.gender {
        object.key("Gender").string(var_31.as_str());
    }
    if let Some(var_32) = &input.home_phone_number {
        object.key("HomePhoneNumber").string(var_32);
    }
    if let Some(var_33) = &input.last_name {
        object.key("LastName").string(var_33);
    }
    if let Some(var_34) = &input.mailing_address {
        let mut object_35 = object.key("MailingAddress").start_object();
        crate::json_ser::serialize_structure_address(&mut object_35, var_34);
        object_35.finish();
    }
    if let Some(var_36) = &input.middle_name {
        object.key("MiddleName").string(var_36);
    }
    if let Some(var_37) = &input.mobile_phone_number {
        object.key("MobilePhoneNumber").string(var_37);
    }
    if let Some(var_38) = &input.party_type {
        object.key("PartyType").string(var_38.as_str());
    }
    if let Some(var_39) = &input.personal_email_address {
        object.key("PersonalEmailAddress").string(var_39);
    }
    if let Some(var_40) = &input.phone_number {
        object.key("PhoneNumber").string(var_40);
    }
    if let Some(var_41) = &input.shipping_address {
        let mut object_42 = object.key("ShippingAddress").start_object();
        crate::json_ser::serialize_structure_address(&mut object_42, var_41);
        object_42.finish();
    }
}

pub fn serialize_structure_delete_integration_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteIntegrationInput,
) {
    if let Some(var_43) = &input.uri {
        object.key("Uri").string(var_43);
    }
}

pub fn serialize_structure_delete_profile_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteProfileInput,
) {
    if let Some(var_44) = &input.profile_id {
        object.key("ProfileId").string(var_44);
    }
}

pub fn serialize_structure_delete_profile_key_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteProfileKeyInput,
) {
    if let Some(var_45) = &input.key_name {
        object.key("KeyName").string(var_45);
    }
    if let Some(var_46) = &input.profile_id {
        object.key("ProfileId").string(var_46);
    }
    if let Some(var_47) = &input.values {
        let mut array_48 = object.key("Values").start_array();
        for item_49 in var_47 {
            {
                array_48.value().string(item_49);
            }
        }
        array_48.finish();
    }
}

pub fn serialize_structure_delete_profile_object_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteProfileObjectInput,
) {
    if let Some(var_50) = &input.object_type_name {
        object.key("ObjectTypeName").string(var_50);
    }
    if let Some(var_51) = &input.profile_id {
        object.key("ProfileId").string(var_51);
    }
    if let Some(var_52) = &input.profile_object_unique_key {
        object.key("ProfileObjectUniqueKey").string(var_52);
    }
}

pub fn serialize_structure_get_integration_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetIntegrationInput,
) {
    if let Some(var_53) = &input.uri {
        object.key("Uri").string(var_53);
    }
}

pub fn serialize_structure_list_account_integrations_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAccountIntegrationsInput,
) {
    if let Some(var_54) = &input.uri {
        object.key("Uri").string(var_54);
    }
}

pub fn serialize_structure_list_profile_objects_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListProfileObjectsInput,
) {
    if let Some(var_55) = &input.object_filter {
        let mut object_56 = object.key("ObjectFilter").start_object();
        crate::json_ser::serialize_structure_object_filter(&mut object_56, var_55);
        object_56.finish();
    }
    if let Some(var_57) = &input.object_type_name {
        object.key("ObjectTypeName").string(var_57);
    }
    if let Some(var_58) = &input.profile_id {
        object.key("ProfileId").string(var_58);
    }
}

pub fn serialize_structure_merge_profiles_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::MergeProfilesInput,
) {
    if let Some(var_59) = &input.field_source_profile_ids {
        let mut object_60 = object.key("FieldSourceProfileIds").start_object();
        crate::json_ser::serialize_structure_field_source_profile_ids(&mut object_60, var_59);
        object_60.finish();
    }
    if let Some(var_61) = &input.main_profile_id {
        object.key("MainProfileId").string(var_61);
    }
    if let Some(var_62) = &input.profile_ids_to_be_merged {
        let mut array_63 = object.key("ProfileIdsToBeMerged").start_array();
        for item_64 in var_62 {
            {
                array_63.value().string(item_64);
            }
        }
        array_63.finish();
    }
}

pub fn serialize_structure_put_integration_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutIntegrationInput,
) {
    if let Some(var_65) = &input.flow_definition {
        let mut object_66 = object.key("FlowDefinition").start_object();
        crate::json_ser::serialize_structure_flow_definition(&mut object_66, var_65);
        object_66.finish();
    }
    if let Some(var_67) = &input.object_type_name {
        object.key("ObjectTypeName").string(var_67);
    }
    if let Some(var_68) = &input.tags {
        let mut object_69 = object.key("Tags").start_object();
        for (key_70, value_71) in var_68 {
            {
                object_69.key(key_70).string(value_71);
            }
        }
        object_69.finish();
    }
    if let Some(var_72) = &input.uri {
        object.key("Uri").string(var_72);
    }
}

pub fn serialize_structure_put_profile_object_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutProfileObjectInput,
) {
    if let Some(var_73) = &input.object {
        object.key("Object").string(var_73);
    }
    if let Some(var_74) = &input.object_type_name {
        object.key("ObjectTypeName").string(var_74);
    }
}

pub fn serialize_structure_put_profile_object_type_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutProfileObjectTypeInput,
) {
    if input.allow_profile_creation {
        object
            .key("AllowProfileCreation")
            .boolean(input.allow_profile_creation);
    }
    if let Some(var_75) = &input.description {
        object.key("Description").string(var_75);
    }
    if let Some(var_76) = &input.encryption_key {
        object.key("EncryptionKey").string(var_76);
    }
    if let Some(var_77) = &input.expiration_days {
        object.key("ExpirationDays").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_77).into()),
        );
    }
    if let Some(var_78) = &input.fields {
        let mut object_79 = object.key("Fields").start_object();
        for (key_80, value_81) in var_78 {
            {
                let mut object_82 = object_79.key(key_80).start_object();
                crate::json_ser::serialize_structure_object_type_field(&mut object_82, value_81);
                object_82.finish();
            }
        }
        object_79.finish();
    }
    if let Some(var_83) = &input.keys {
        let mut object_84 = object.key("Keys").start_object();
        for (key_85, value_86) in var_83 {
            {
                let mut array_87 = object_84.key(key_85).start_array();
                for item_88 in value_86 {
                    {
                        let mut object_89 = array_87.value().start_object();
                        crate::json_ser::serialize_structure_object_type_key(
                            &mut object_89,
                            item_88,
                        );
                        object_89.finish();
                    }
                }
                array_87.finish();
            }
        }
        object_84.finish();
    }
    if let Some(var_90) = &input.tags {
        let mut object_91 = object.key("Tags").start_object();
        for (key_92, value_93) in var_90 {
            {
                object_91.key(key_92).string(value_93);
            }
        }
        object_91.finish();
    }
    if let Some(var_94) = &input.template_id {
        object.key("TemplateId").string(var_94);
    }
}

pub fn serialize_structure_search_profiles_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SearchProfilesInput,
) {
    if let Some(var_95) = &input.key_name {
        object.key("KeyName").string(var_95);
    }
    if let Some(var_96) = &input.values {
        let mut array_97 = object.key("Values").start_array();
        for item_98 in var_96 {
            {
                array_97.value().string(item_98);
            }
        }
        array_97.finish();
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_99) = &input.tags {
        let mut object_100 = object.key("tags").start_object();
        for (key_101, value_102) in var_99 {
            {
                object_100.key(key_101).string(value_102);
            }
        }
        object_100.finish();
    }
}

pub fn serialize_structure_update_domain_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDomainInput,
) {
    if let Some(var_103) = &input.dead_letter_queue_url {
        object.key("DeadLetterQueueUrl").string(var_103);
    }
    if let Some(var_104) = &input.default_encryption_key {
        object.key("DefaultEncryptionKey").string(var_104);
    }
    if let Some(var_105) = &input.default_expiration_days {
        object.key("DefaultExpirationDays").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_105).into()),
        );
    }
    if let Some(var_106) = &input.matching {
        let mut object_107 = object.key("Matching").start_object();
        crate::json_ser::serialize_structure_matching_request(&mut object_107, var_106);
        object_107.finish();
    }
    if let Some(var_108) = &input.tags {
        let mut object_109 = object.key("Tags").start_object();
        for (key_110, value_111) in var_108 {
            {
                object_109.key(key_110).string(value_111);
            }
        }
        object_109.finish();
    }
}

pub fn serialize_structure_update_profile_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateProfileInput,
) {
    if let Some(var_112) = &input.account_number {
        object.key("AccountNumber").string(var_112);
    }
    if let Some(var_113) = &input.additional_information {
        object.key("AdditionalInformation").string(var_113);
    }
    if let Some(var_114) = &input.address {
        let mut object_115 = object.key("Address").start_object();
        crate::json_ser::serialize_structure_update_address(&mut object_115, var_114);
        object_115.finish();
    }
    if let Some(var_116) = &input.attributes {
        let mut object_117 = object.key("Attributes").start_object();
        for (key_118, value_119) in var_116 {
            {
                object_117.key(key_118).string(value_119);
            }
        }
        object_117.finish();
    }
    if let Some(var_120) = &input.billing_address {
        let mut object_121 = object.key("BillingAddress").start_object();
        crate::json_ser::serialize_structure_update_address(&mut object_121, var_120);
        object_121.finish();
    }
    if let Some(var_122) = &input.birth_date {
        object.key("BirthDate").string(var_122);
    }
    if let Some(var_123) = &input.business_email_address {
        object.key("BusinessEmailAddress").string(var_123);
    }
    if let Some(var_124) = &input.business_name {
        object.key("BusinessName").string(var_124);
    }
    if let Some(var_125) = &input.business_phone_number {
        object.key("BusinessPhoneNumber").string(var_125);
    }
    if let Some(var_126) = &input.email_address {
        object.key("EmailAddress").string(var_126);
    }
    if let Some(var_127) = &input.first_name {
        object.key("FirstName").string(var_127);
    }
    if let Some(var_128) = &input.gender {
        object.key("Gender").string(var_128.as_str());
    }
    if let Some(var_129) = &input.home_phone_number {
        object.key("HomePhoneNumber").string(var_129);
    }
    if let Some(var_130) = &input.last_name {
        object.key("LastName").string(var_130);
    }
    if let Some(var_131) = &input.mailing_address {
        let mut object_132 = object.key("MailingAddress").start_object();
        crate::json_ser::serialize_structure_update_address(&mut object_132, var_131);
        object_132.finish();
    }
    if let Some(var_133) = &input.middle_name {
        object.key("MiddleName").string(var_133);
    }
    if let Some(var_134) = &input.mobile_phone_number {
        object.key("MobilePhoneNumber").string(var_134);
    }
    if let Some(var_135) = &input.party_type {
        object.key("PartyType").string(var_135.as_str());
    }
    if let Some(var_136) = &input.personal_email_address {
        object.key("PersonalEmailAddress").string(var_136);
    }
    if let Some(var_137) = &input.phone_number {
        object.key("PhoneNumber").string(var_137);
    }
    if let Some(var_138) = &input.profile_id {
        object.key("ProfileId").string(var_138);
    }
    if let Some(var_139) = &input.shipping_address {
        let mut object_140 = object.key("ShippingAddress").start_object();
        crate::json_ser::serialize_structure_update_address(&mut object_140, var_139);
        object_140.finish();
    }
}

pub fn serialize_structure_matching_request(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MatchingRequest,
) {
    if let Some(var_141) = &input.enabled {
        object.key("Enabled").boolean(*var_141);
    }
}

pub fn serialize_structure_address(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Address,
) {
    if let Some(var_142) = &input.address1 {
        object.key("Address1").string(var_142);
    }
    if let Some(var_143) = &input.address2 {
        object.key("Address2").string(var_143);
    }
    if let Some(var_144) = &input.address3 {
        object.key("Address3").string(var_144);
    }
    if let Some(var_145) = &input.address4 {
        object.key("Address4").string(var_145);
    }
    if let Some(var_146) = &input.city {
        object.key("City").string(var_146);
    }
    if let Some(var_147) = &input.county {
        object.key("County").string(var_147);
    }
    if let Some(var_148) = &input.state {
        object.key("State").string(var_148);
    }
    if let Some(var_149) = &input.province {
        object.key("Province").string(var_149);
    }
    if let Some(var_150) = &input.country {
        object.key("Country").string(var_150);
    }
    if let Some(var_151) = &input.postal_code {
        object.key("PostalCode").string(var_151);
    }
}

pub fn serialize_structure_object_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ObjectFilter,
) {
    if let Some(var_152) = &input.key_name {
        object.key("KeyName").string(var_152);
    }
    if let Some(var_153) = &input.values {
        let mut array_154 = object.key("Values").start_array();
        for item_155 in var_153 {
            {
                array_154.value().string(item_155);
            }
        }
        array_154.finish();
    }
}

pub fn serialize_structure_field_source_profile_ids(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FieldSourceProfileIds,
) {
    if let Some(var_156) = &input.account_number {
        object.key("AccountNumber").string(var_156);
    }
    if let Some(var_157) = &input.additional_information {
        object.key("AdditionalInformation").string(var_157);
    }
    if let Some(var_158) = &input.party_type {
        object.key("PartyType").string(var_158);
    }
    if let Some(var_159) = &input.business_name {
        object.key("BusinessName").string(var_159);
    }
    if let Some(var_160) = &input.first_name {
        object.key("FirstName").string(var_160);
    }
    if let Some(var_161) = &input.middle_name {
        object.key("MiddleName").string(var_161);
    }
    if let Some(var_162) = &input.last_name {
        object.key("LastName").string(var_162);
    }
    if let Some(var_163) = &input.birth_date {
        object.key("BirthDate").string(var_163);
    }
    if let Some(var_164) = &input.gender {
        object.key("Gender").string(var_164);
    }
    if let Some(var_165) = &input.phone_number {
        object.key("PhoneNumber").string(var_165);
    }
    if let Some(var_166) = &input.mobile_phone_number {
        object.key("MobilePhoneNumber").string(var_166);
    }
    if let Some(var_167) = &input.home_phone_number {
        object.key("HomePhoneNumber").string(var_167);
    }
    if let Some(var_168) = &input.business_phone_number {
        object.key("BusinessPhoneNumber").string(var_168);
    }
    if let Some(var_169) = &input.email_address {
        object.key("EmailAddress").string(var_169);
    }
    if let Some(var_170) = &input.personal_email_address {
        object.key("PersonalEmailAddress").string(var_170);
    }
    if let Some(var_171) = &input.business_email_address {
        object.key("BusinessEmailAddress").string(var_171);
    }
    if let Some(var_172) = &input.address {
        object.key("Address").string(var_172);
    }
    if let Some(var_173) = &input.shipping_address {
        object.key("ShippingAddress").string(var_173);
    }
    if let Some(var_174) = &input.mailing_address {
        object.key("MailingAddress").string(var_174);
    }
    if let Some(var_175) = &input.billing_address {
        object.key("BillingAddress").string(var_175);
    }
    if let Some(var_176) = &input.attributes {
        let mut object_177 = object.key("Attributes").start_object();
        for (key_178, value_179) in var_176 {
            {
                object_177.key(key_178).string(value_179);
            }
        }
        object_177.finish();
    }
}

pub fn serialize_structure_flow_definition(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FlowDefinition,
) {
    if let Some(var_180) = &input.description {
        object.key("Description").string(var_180);
    }
    if let Some(var_181) = &input.flow_name {
        object.key("FlowName").string(var_181);
    }
    if let Some(var_182) = &input.kms_arn {
        object.key("KmsArn").string(var_182);
    }
    if let Some(var_183) = &input.source_flow_config {
        let mut object_184 = object.key("SourceFlowConfig").start_object();
        crate::json_ser::serialize_structure_source_flow_config(&mut object_184, var_183);
        object_184.finish();
    }
    if let Some(var_185) = &input.tasks {
        let mut array_186 = object.key("Tasks").start_array();
        for item_187 in var_185 {
            {
                let mut object_188 = array_186.value().start_object();
                crate::json_ser::serialize_structure_task(&mut object_188, item_187);
                object_188.finish();
            }
        }
        array_186.finish();
    }
    if let Some(var_189) = &input.trigger_config {
        let mut object_190 = object.key("TriggerConfig").start_object();
        crate::json_ser::serialize_structure_trigger_config(&mut object_190, var_189);
        object_190.finish();
    }
}

pub fn serialize_structure_object_type_field(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ObjectTypeField,
) {
    if let Some(var_191) = &input.source {
        object.key("Source").string(var_191);
    }
    if let Some(var_192) = &input.target {
        object.key("Target").string(var_192);
    }
    if let Some(var_193) = &input.content_type {
        object.key("ContentType").string(var_193.as_str());
    }
}

pub fn serialize_structure_object_type_key(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ObjectTypeKey,
) {
    if let Some(var_194) = &input.standard_identifiers {
        let mut array_195 = object.key("StandardIdentifiers").start_array();
        for item_196 in var_194 {
            {
                array_195.value().string(item_196.as_str());
            }
        }
        array_195.finish();
    }
    if let Some(var_197) = &input.field_names {
        let mut array_198 = object.key("FieldNames").start_array();
        for item_199 in var_197 {
            {
                array_198.value().string(item_199);
            }
        }
        array_198.finish();
    }
}

pub fn serialize_structure_update_address(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateAddress,
) {
    if let Some(var_200) = &input.address1 {
        object.key("Address1").string(var_200);
    }
    if let Some(var_201) = &input.address2 {
        object.key("Address2").string(var_201);
    }
    if let Some(var_202) = &input.address3 {
        object.key("Address3").string(var_202);
    }
    if let Some(var_203) = &input.address4 {
        object.key("Address4").string(var_203);
    }
    if let Some(var_204) = &input.city {
        object.key("City").string(var_204);
    }
    if let Some(var_205) = &input.county {
        object.key("County").string(var_205);
    }
    if let Some(var_206) = &input.state {
        object.key("State").string(var_206);
    }
    if let Some(var_207) = &input.province {
        object.key("Province").string(var_207);
    }
    if let Some(var_208) = &input.country {
        object.key("Country").string(var_208);
    }
    if let Some(var_209) = &input.postal_code {
        object.key("PostalCode").string(var_209);
    }
}

pub fn serialize_structure_source_flow_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SourceFlowConfig,
) {
    if let Some(var_210) = &input.connector_profile_name {
        object.key("ConnectorProfileName").string(var_210);
    }
    if let Some(var_211) = &input.connector_type {
        object.key("ConnectorType").string(var_211.as_str());
    }
    if let Some(var_212) = &input.incremental_pull_config {
        let mut object_213 = object.key("IncrementalPullConfig").start_object();
        crate::json_ser::serialize_structure_incremental_pull_config(&mut object_213, var_212);
        object_213.finish();
    }
    if let Some(var_214) = &input.source_connector_properties {
        let mut object_215 = object.key("SourceConnectorProperties").start_object();
        crate::json_ser::serialize_structure_source_connector_properties(&mut object_215, var_214);
        object_215.finish();
    }
}

pub fn serialize_structure_task(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Task,
) {
    if let Some(var_216) = &input.connector_operator {
        let mut object_217 = object.key("ConnectorOperator").start_object();
        crate::json_ser::serialize_structure_connector_operator(&mut object_217, var_216);
        object_217.finish();
    }
    if let Some(var_218) = &input.destination_field {
        object.key("DestinationField").string(var_218);
    }
    if let Some(var_219) = &input.source_fields {
        let mut array_220 = object.key("SourceFields").start_array();
        for item_221 in var_219 {
            {
                array_220.value().string(item_221);
            }
        }
        array_220.finish();
    }
    if let Some(var_222) = &input.task_properties {
        let mut object_223 = object.key("TaskProperties").start_object();
        for (key_224, value_225) in var_222 {
            {
                object_223.key(key_224.as_str()).string(value_225);
            }
        }
        object_223.finish();
    }
    if let Some(var_226) = &input.task_type {
        object.key("TaskType").string(var_226.as_str());
    }
}

pub fn serialize_structure_trigger_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TriggerConfig,
) {
    if let Some(var_227) = &input.trigger_type {
        object.key("TriggerType").string(var_227.as_str());
    }
    if let Some(var_228) = &input.trigger_properties {
        let mut object_229 = object.key("TriggerProperties").start_object();
        crate::json_ser::serialize_structure_trigger_properties(&mut object_229, var_228);
        object_229.finish();
    }
}

pub fn serialize_structure_incremental_pull_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IncrementalPullConfig,
) {
    if let Some(var_230) = &input.datetime_type_field_name {
        object.key("DatetimeTypeFieldName").string(var_230);
    }
}

pub fn serialize_structure_source_connector_properties(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SourceConnectorProperties,
) {
    if let Some(var_231) = &input.marketo {
        let mut object_232 = object.key("Marketo").start_object();
        crate::json_ser::serialize_structure_marketo_source_properties(&mut object_232, var_231);
        object_232.finish();
    }
    if let Some(var_233) = &input.s3 {
        let mut object_234 = object.key("S3").start_object();
        crate::json_ser::serialize_structure_s3_source_properties(&mut object_234, var_233);
        object_234.finish();
    }
    if let Some(var_235) = &input.salesforce {
        let mut object_236 = object.key("Salesforce").start_object();
        crate::json_ser::serialize_structure_salesforce_source_properties(&mut object_236, var_235);
        object_236.finish();
    }
    if let Some(var_237) = &input.service_now {
        let mut object_238 = object.key("ServiceNow").start_object();
        crate::json_ser::serialize_structure_service_now_source_properties(
            &mut object_238,
            var_237,
        );
        object_238.finish();
    }
    if let Some(var_239) = &input.zendesk {
        let mut object_240 = object.key("Zendesk").start_object();
        crate::json_ser::serialize_structure_zendesk_source_properties(&mut object_240, var_239);
        object_240.finish();
    }
}

pub fn serialize_structure_connector_operator(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ConnectorOperator,
) {
    if let Some(var_241) = &input.marketo {
        object.key("Marketo").string(var_241.as_str());
    }
    if let Some(var_242) = &input.s3 {
        object.key("S3").string(var_242.as_str());
    }
    if let Some(var_243) = &input.salesforce {
        object.key("Salesforce").string(var_243.as_str());
    }
    if let Some(var_244) = &input.service_now {
        object.key("ServiceNow").string(var_244.as_str());
    }
    if let Some(var_245) = &input.zendesk {
        object.key("Zendesk").string(var_245.as_str());
    }
}

pub fn serialize_structure_trigger_properties(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TriggerProperties,
) {
    if let Some(var_246) = &input.scheduled {
        let mut object_247 = object.key("Scheduled").start_object();
        crate::json_ser::serialize_structure_scheduled_trigger_properties(&mut object_247, var_246);
        object_247.finish();
    }
}

pub fn serialize_structure_marketo_source_properties(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MarketoSourceProperties,
) {
    if let Some(var_248) = &input.object {
        object.key("Object").string(var_248);
    }
}

pub fn serialize_structure_s3_source_properties(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3SourceProperties,
) {
    if let Some(var_249) = &input.bucket_name {
        object.key("BucketName").string(var_249);
    }
    if let Some(var_250) = &input.bucket_prefix {
        object.key("BucketPrefix").string(var_250);
    }
}

pub fn serialize_structure_salesforce_source_properties(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SalesforceSourceProperties,
) {
    if let Some(var_251) = &input.object {
        object.key("Object").string(var_251);
    }
    if input.enable_dynamic_field_update {
        object
            .key("EnableDynamicFieldUpdate")
            .boolean(input.enable_dynamic_field_update);
    }
    if input.include_deleted_records {
        object
            .key("IncludeDeletedRecords")
            .boolean(input.include_deleted_records);
    }
}

pub fn serialize_structure_service_now_source_properties(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ServiceNowSourceProperties,
) {
    if let Some(var_252) = &input.object {
        object.key("Object").string(var_252);
    }
}

pub fn serialize_structure_zendesk_source_properties(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ZendeskSourceProperties,
) {
    if let Some(var_253) = &input.object {
        object.key("Object").string(var_253);
    }
}

pub fn serialize_structure_scheduled_trigger_properties(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ScheduledTriggerProperties,
) {
    if let Some(var_254) = &input.schedule_expression {
        object.key("ScheduleExpression").string(var_254);
    }
    if let Some(var_255) = &input.data_pull_mode {
        object.key("DataPullMode").string(var_255.as_str());
    }
    if let Some(var_256) = &input.schedule_start_time {
        object
            .key("ScheduleStartTime")
            .instant(var_256, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_257) = &input.schedule_end_time {
        object
            .key("ScheduleEndTime")
            .instant(var_257, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_258) = &input.timezone {
        object.key("Timezone").string(var_258);
    }
    if let Some(var_259) = &input.schedule_offset {
        object.key("ScheduleOffset").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_259).into()),
        );
    }
    if let Some(var_260) = &input.first_execution_from {
        object
            .key("FirstExecutionFrom")
            .instant(var_260, smithy_types::instant::Format::EpochSeconds);
    }
}