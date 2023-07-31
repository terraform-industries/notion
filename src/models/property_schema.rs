use super::properties::NumberFormat;
use serde::{Deserialize, Serialize};

#[derive(Eq, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PropertySchema {
    Title {},
    RichText {},
    Number {
        #[serde(skip_serializing_if = "Option::is_none")]
        format: Option<NumberFormat>,
    },
    Select {
        options: Option<Vec<SelectOptionSchema>>,
    },
    MultiSelect {
        options: Option<Vec<SelectOptionSchema>>,
    },
    Date {},
    People {},
    Files {},
    Checkbox {},
    Url {},
    Email {},
    PhoneNumber {},
    Formula {
        expression: String,
    },
    Relation {
        database_id: String,
        #[serde(rename = "type")]
        relation_type: RelationType,
    },
    Rollup {
        #[serde(skip_serializing_if = "Option::is_none")]
        relation_property_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        relation_property_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        rollup_property_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        rollup_property_id: Option<String>,
        function: RollupFunction,
    },
    CreatedTime {},
    CreatedBy {},
    LastEditedTime {},
    LastEditedBy {},
}

#[derive(Eq, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct SelectOptionSchema {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
}

#[derive(Eq, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RelationType {
    SingleProperty,
    DualProperty,
}

#[derive(Eq, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RollupFunction {
    CountAll,
    CountValues,
    CountUniqueValues,
    CountEmpty,
    CountNotEmpty,
    PercentEmpty,
    PercentNotEmpty,
    Sum,
    Average,
    Median,
    Min,
    Max,
    Range,
    ShowOriginal,
}

#[cfg(test)]
mod test {
    use crate::models::{properties::NumberFormat, property_schema::PropertySchema};

    #[test]
    fn property_schema_serialization() {
        let schema = PropertySchema::Title {};
        let serialized = serde_json::to_string(&schema).unwrap();
        assert_eq!(serialized, r#"{"title":{}}"#);

        let schema = PropertySchema::Number {
            format: Some(NumberFormat::Dollar),
        };
        let serialized = serde_json::to_string(&schema).unwrap();
        assert_eq!(serialized, r#"{"number":{"format":"dollar"}}"#);

        let schema = PropertySchema::Relation {
            database_id: "foobar".to_string(),
            relation_type: crate::models::property_schema::RelationType::DualProperty,
        };
        let serialized = serde_json::to_string(&schema).unwrap();
        assert_eq!(
            serialized,
            r#"{"relation":{"database_id":"foobar","type":"dual_property"}}"#
        );
    }
}
