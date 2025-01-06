use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct Sample {
    #[builder(
        custom(
            type = impl
            conjure_object::serde::Serialize,
            convert = |v|conjure_object::Any::new(v).expect("value failed to serialize")
        )
    )]
    value: conjure_object::Any,
    time: conjure_object::DateTime<conjure_object::Utc>,
    #[builder(default, into)]
    trace_id: Option<super::TraceId>,
}
impl Sample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(
        value: impl conjure_object::serde::Serialize,
        time: conjure_object::DateTime<conjure_object::Utc>,
    ) -> Self {
        Self::builder().value(value).time(time).build()
    }
    ///Exact value of this metric sample
    #[inline]
    pub fn value(&self) -> &conjure_object::Any {
        &self.value
    }
    ///RFC3339Nano UTC datetime string of when the sample was taken
    #[inline]
    pub fn time(&self) -> conjure_object::DateTime<conjure_object::Utc> {
        self.time
    }
    ///Zipkin trace id associated with this sample, if available
    #[inline]
    pub fn trace_id(&self) -> Option<&super::TraceId> {
        self.trace_id.as_ref().map(|o| &*o)
    }
}
impl ser::Serialize for Sample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 2usize;
        let skip_trace_id = self.trace_id.is_none();
        if !skip_trace_id {
            size += 1;
        }
        let mut s = s.serialize_struct("Sample", size)?;
        s.serialize_field("value", &self.value)?;
        s.serialize_field("time", &self.time)?;
        if skip_trace_id {
            s.skip_field("traceId")?;
        } else {
            s.serialize_field("traceId", &self.trace_id)?;
        }
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for Sample {
    fn deserialize<D>(d: D) -> Result<Sample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("Sample", &["value", "time", "traceId"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = Sample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<Sample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut value = None;
        let mut time = None;
        let mut trace_id = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Value => value = Some(map_.next_value()?),
                Field_::Time => time = Some(map_.next_value()?),
                Field_::TraceId => trace_id = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let value = match value {
            Some(v) => v,
            None => return Err(de::Error::missing_field("value")),
        };
        let time = match time {
            Some(v) => v,
            None => return Err(de::Error::missing_field("time")),
        };
        let trace_id = match trace_id {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(Sample { value, time, trace_id })
    }
}
enum Field_ {
    Value,
    Time,
    TraceId,
    Unknown_,
}
impl<'de> de::Deserialize<'de> for Field_ {
    fn deserialize<D>(d: D) -> Result<Field_, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_str(FieldVisitor_)
    }
}
struct FieldVisitor_;
impl<'de> de::Visitor<'de> for FieldVisitor_ {
    type Value = Field_;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("string")
    }
    fn visit_str<E>(self, value: &str) -> Result<Field_, E>
    where
        E: de::Error,
    {
        let v = match value {
            "value" => Field_::Value,
            "time" => Field_::Time,
            "traceId" => Field_::TraceId,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
