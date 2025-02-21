use crate::config::IntegrationConfig;
use crate::forwarding::GenericQueueForwarder;
use crate::{
    config::{ForwardDestination, VerificationScheme},
    forwarding::{Forwarder, ForwardingMethod},
    verification::{NoVerifier, SvixVerifier, VerificationMethod, Verifier},
};
use anyhow::Result;
use axum::{
    async_trait,
    body::{Bytes, HttpBody},
    extract::FromRequest,
    BoxError,
};
use http::{HeaderMap, HeaderValue, Request};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, marker::PhantomData, sync::Arc};

#[derive(Clone, Debug)]
/// The [`InternalState`] is passed to the Axum route and is used to map the "IntegrationId" in the
/// URL to the configured [`Verifier`] and [`Forwarder`] variants.
pub struct InternalState(pub HashMap<IntegrationId, IntegrationState>);
impl InternalState {
    pub async fn from_routes(routes: &[IntegrationConfig]) -> Result<Self> {
        let mut state_map = HashMap::new();

        for cfg in routes {
            let verifier = match &cfg.verification {
                VerificationScheme::None => NoVerifier.into(),
                VerificationScheme::Svix { secret } => SvixVerifier::new(Arc::new(
                    svix::webhooks::Webhook::new(
                        &secret.to_secret().expect("Error reading secret"),
                    )
                    .expect("Invalid Svix secret"),
                ))
                .into(),
            };

            let forwarder = match &cfg.destination {
                ForwardDestination::GCPPubSub(sender_cfg) => {
                    GenericQueueForwarder::from_gcp_pupsub_cfg(sender_cfg.clone()).await?
                }
                ForwardDestination::RabbitMQ(sender_cfg) => {
                    GenericQueueForwarder::from_rabbitmq_cfg(sender_cfg.clone()).await?
                }
                ForwardDestination::Redis(sender_cfg) => {
                    GenericQueueForwarder::from_redis_cfg(sender_cfg.clone()).await?
                }
                ForwardDestination::SQS(sender_cfg) => {
                    GenericQueueForwarder::from_sqs_cfg(sender_cfg.clone()).await?
                }
            }
            .into();

            state_map.insert(
                cfg.name.clone(),
                IntegrationState {
                    verifier,
                    forwarder,
                },
            );
        }

        Ok(InternalState(state_map))
    }
}

/// Each [`IntegrationId`] is a valid route for webhooks to be dispatched to managed by this server,
/// and each [`IntegrationId`] has an associated configuration which defines how the webhook is
/// verified (the [`VerificationScheme`]) and where the webhook is routed to once it is verified
/// (the [`ForwardDestination`]).
///
/// Internally it is also associated with an [`IntegrationState`] which will contain the necessary
/// members to actually perform these actions eg. a handle to a [`FutureProducer`] instead of simply
/// the address(es) of the Kafka bootstrap server(s).
///
/// This type is simply a wrapper for a [`String`] which *should* be safe to use in a URL. If it is
/// not a valid path component for a URL, then the [`IntegrationId`] will never receive any
/// webhooks. However, for simplicity, the inner [`String`] is not validated for URL-safety at this
/// time.
#[repr(transparent)]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IntegrationId(String);

impl AsRef<str> for IntegrationId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// The [`IntegrationState`] is a struct which is only able to be created via conversion from a
/// [`IntegrationConfig`]. This struct is what is associated with an `[IntegrationId`] internally
/// after the configuration has been read.
///
/// What distinguishes it from the [`IntegrationConfig`] is that it contains the necessary members
/// for validating and forwarding a webhook instead of just containing the definition of how to
/// derive these necessary members.
#[derive(Clone, Debug)]
pub struct IntegrationState {
    pub verifier: Verifier,
    pub forwarder: Forwarder,
}

/// Any arbitrary HTTP request which is not a webhook dispatched by Svix may also have arbitrary
/// validation associated with it by means of custom JavaScript. This JavaScript is evaluated by
/// the Deno JS runtime.
///
/// The convention of the contained JavaScript is that it should include a function as a default
/// export which takes a single input. This input will be a JSON object including all headers that
/// came from the request in a map and the payload verbatim. This exported function must return a
/// `bool` for the associated [`IntegrationId`]'s route to function in any capacity.
///
/// Should the `handler` function return `true`, then the request is deemed a valid webhook as per
/// the user's specifications and the webhook is then forwarded as with the Svix scheme via the
/// configured [`ForwardDestination`].
///
/// Should a `handler` return `false`, then the request is either silently discarded or logged at
/// the warning level depending on the value of `log_on_invalid` in the [`crate::config::Config`].
///
/// Should a `handler` throw an error or return a value that is not a `bool`, then an error will
/// be logged and the request is discarded.
#[repr(transparent)]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct JsCode(pub String);

impl From<String> for JsCode {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// The [`RequestFromParts`] is a structure consisting of all relevant parts of the HTTP request to
/// be validated by a [`Verifier`] implementor. This is to be immediately converted into the struct
/// [`SerializableRequest<Unvalidated>`] via its [`FromRequest`] implementation.
///
/// NOTE: This struct is never to be used directly unless by proxy of the aforementioned impl of
/// [`FromRequest`]. It's simply used as any easy way to implement [`FromRequest`] via a macro .
#[derive(Clone, Debug, FromRequest)]
pub struct RequestFromParts {
    headers: HeaderMap,
    payload: Bytes,
}

/// A simple marker trait to denote the state of a [`SerializableRequest`]. The only way to publicly
/// construct any [`SerializableRequest<Validated>`]s  is via the associated method on unvalidated
/// request's, [`SerializableRequest<Unvalidated>::validate`].
pub trait RequestState {}

#[derive(Clone, Copy, Debug)]
pub struct Unvalidated;
impl RequestState for Unvalidated {}

#[derive(Clone, Copy, Debug)]
pub struct Validated;
impl RequestState for Validated {}

/// This intermediary representation is necessary because it is preferable to serialize the headers
/// and/or body as a [`String`] over bytes when dealing with some [`VerificationMethod`]s and some
/// [`ForwardingMethod`]s. This struct represents both the headers and body as enums which allow for
/// either textual representations or byte representations when [`Serialize`]d via [`serde`].
///
/// On trying to convert a [`Standard`] variant into a [`StringSerializable`] variant, HTTP headers
/// will be represented textually if and only if they are completely ASCII, while any bodies will
/// attempt to be read as UTF-8 before falling back to bytes.
///
/// NOTE: This conversion *should* be lazy. The [`String`] variant are only acceptable in a subset
/// of all cases, so lazy-conversion will prevent needless conversion back and forth. You may check
/// whether the conversion is required and/or helpful with [`VerificationMethod::want_string_rep`]
/// or [`VerificationMethod::need_string_rep`] plus the [`ForwardingMethod`] equivalents.
///
/// The intended course of action is to attempt to convert to string-serializable variants of the
/// header map and the body immediately if either of the aforementioned methods are true --  but
/// only returning an [`Err`] response if it *needs* it. Then, if the validation is a success (see
/// [`SerializableRequest<Unvalidated>::validate`] and a validated equivalent is returned, then the
/// same checks are to be performed, but with the [`ForwardingMethod`] methods before being sent to
/// the appropriate [`ForwardingMethod`] implementor.
#[derive(Clone, Debug, Serialize)]
pub struct SerializableRequest<S: RequestState> {
    headers: SerializableHeaderMap,
    payload: SerializablePayload,

    #[serde(skip)]
    _pd: PhantomData<S>,
}

impl<S: RequestState> SerializableRequest<S> {
    pub fn headers(&self) -> &SerializableHeaderMap {
        &self.headers
    }

    pub fn payload(&self) -> &SerializablePayload {
        &self.payload
    }
}

impl From<RequestFromParts> for SerializableRequest<Unvalidated> {
    fn from(value: RequestFromParts) -> Self {
        Self {
            headers: SerializableHeaderMap::Standard(value.headers),
            payload: SerializablePayload::Standard(value.payload.to_vec()),

            _pd: PhantomData,
        }
    }
}

#[async_trait]
impl<S, B> FromRequest<S, B> for SerializableRequest<Unvalidated>
where
    S: Send + Sync,
    B: HttpBody + Send + Sync + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = <RequestFromParts as FromRequest<S, B>>::Rejection;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        RequestFromParts::from_request(req, state)
            .await
            .map(Into::into)
    }
}

impl SerializableRequest<Unvalidated> {
    /// Given a specific validator
    pub async fn validate<V: VerificationMethod>(
        mut self,
        verifier: &V,
    ) -> Result<SerializableRequest<Validated>, http::StatusCode> {
        // Do relevant conversions to [`String`] representaitons if wanted/needed
        match (verifier.want_string_rep(), verifier.need_string_rep()) {
            // Needed
            (true, true) | (false, true) => {
                self.headers = self
                    .headers
                    .try_to_string()
                    .map_err(|_| http::StatusCode::BAD_REQUEST)?;
                self.payload = self
                    .payload
                    .try_to_string()
                    .map_err(|_| http::StatusCode::BAD_REQUEST)?;
            }

            // Wanted, but not needed
            (true, false) => {
                self.headers = match self.headers.try_to_string() {
                    Ok(h) => h,
                    Err(h) => h,
                };

                self.payload = match self.payload.try_to_string() {
                    Ok(p) => p,
                    Err(p) => p,
                };
            }

            // Not wanted
            (false, false) => {}
        };

        // FIXME: No cloning
        // Then actually use the [`VerificationMethod`] implementor.
        match verifier.validate(self.clone()).await {
            Ok(true) => Ok(SerializableRequest::<Validated> {
                headers: self.headers,
                payload: self.payload,

                _pd: PhantomData,
            }),

            Ok(false) => {
                // FIXME: Read config to know whether to log
                Err(http::StatusCode::BAD_REQUEST)
            }

            Err(e) => {
                tracing::error!("Error validating request: {}", e);
                println!("Error validating request: {}", e);
                Err(http::StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

impl SerializableRequest<Validated> {
    pub async fn forward<F: ForwardingMethod>(self, f: &F) -> http::StatusCode {
        match f.forward(self).await {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Error forwarding request: {}", e);
                http::StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum SerializableHeaderMap {
    Standard(HeaderMap),
    StringSerializable(HashMap<String, String>),
}

impl<'a> IntoIterator for &'a SerializableHeaderMap {
    type Item = (&'a str, &'a [u8]);
    type IntoIter = SerializableHeaderMapIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            SerializableHeaderMap::Standard(hm) => SerializableHeaderMapIter::HeaderMap(hm.iter()),
            SerializableHeaderMap::StringSerializable(hm) => {
                SerializableHeaderMapIter::HashMap(hm.iter())
            }
        }
    }
}

impl SerializableHeaderMap {
    pub fn try_to_string(self) -> Result<Self, Self> {
        match self {
            Self::Standard(header_map) => Ok(Self::StringSerializable(
                header_map
                    .iter()
                    .map(|(name, value)| Ok((name.as_str().to_owned(), value.to_str()?.to_owned())))
                    .collect::<Result<HashMap<String, String>>>()
                    .map_err(|_| Self::Standard(header_map))?,
            )),
            Self::StringSerializable(hash_map) => Ok(Self::StringSerializable(hash_map)),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Standard(m) => m.len(),
            Self::StringSerializable(m) => m.len(),
        }
    }
}

/// Serialize is not implemented on [`HeaderMap`]s themselves, so custom serialization is required.
impl Serialize for SerializableHeaderMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Standard(header_map) => header_map
                .iter()
                .map(|(name, value)| (name.as_str().to_owned(), value.as_bytes().to_vec()))
                .collect::<HashMap<String, Vec<u8>>>()
                .serialize(serializer),
            Self::StringSerializable(hash_map) => hash_map.serialize(serializer),
        }
    }
}

pub enum SerializableHeaderMapIter<'a> {
    HeaderMap(http::header::Iter<'a, HeaderValue>),
    HashMap(std::collections::hash_map::Iter<'a, String, String>),
}

impl<'a> Iterator for SerializableHeaderMapIter<'a> {
    type Item = (&'a str, &'a [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::HeaderMap(hm) => hm.next().map(|(k, v)| (k.as_str(), v.as_bytes())),
            Self::HashMap(hm) => hm.next().map(|(k, v)| (k.as_str(), v.as_bytes())),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum SerializablePayload {
    Standard(Vec<u8>),
    StringSerializable(String),
}

impl SerializablePayload {
    fn try_to_string(self) -> Result<Self, Self> {
        match self {
            Self::Standard(v) => Ok(Self::StringSerializable(
                String::from_utf8(v).map_err(|e| Self::Standard(e.into_bytes()))?,
            )),
            Self::StringSerializable(s) => Ok(Self::StringSerializable(s)),
        }
    }
}
