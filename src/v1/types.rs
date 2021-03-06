//! Types returned by the API.
//!
//! This module contains all the types required by the API to enable an easier use of it.

use std::collections::{btree_set, BTreeSet};
use std::slice::Iter;
use std::result::Result as StdResult;

use chrono::{DateTime, UTC, NaiveDate};
#[cfg(feature = "json-types")]
use rustc_serialize::json;

use dto::{UserDTO, FromDTO, FromDTOError, ScopeDTO as Scope, ClientInfoDTO, TransactionDTO,
          ProfileDTO, PendingFriendRequestDTO};
use utils::{WalletAddress, Amount, Address};

/// Information about the API client.
#[derive(Clone, Debug)]
pub struct ClientInfo {
    id: String,
    secret: String,
    scopes: Vec<Scope>,
    request_limit: Option<usize>,
}

impl ClientInfo {
    /// Gets the client ID.
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// Gets the client secret.
    pub fn get_secret(&self) -> &str {
        &self.secret
    }

    /// Gets an iterator through the scopes valid for the client.
    pub fn scopes(&self) -> Iter<Scope> {
        self.scopes.iter()
    }

    /// Gets the request limit for the client.
    pub fn get_request_limit(&self) -> Option<usize> {
        self.request_limit
    }
}

#[cfg(feature = "json-types")]
impl json::ToJson for ClientInfo {
    fn to_json(&self) -> json::Json {
        let mut object = json::Object::new();
        let _ = object.insert(String::from("id"), self.id.to_json());
        let _ = object.insert(String::from("secret"), self.secret.to_json());
        let mut json_scopes = Vec::with_capacity(self.scopes.len());
        for scope in self.scopes.iter() {
            json_scopes.push(format!("{}", scope).to_json());
        }
        let _ = object.insert(String::from("scopes"), json_scopes.to_json());
        let _ = object.insert(String::from("request_limit"), self.request_limit.to_json());

        json::Json::Object(object)
    }
}

impl FromDTO<ClientInfoDTO> for ClientInfo {
    fn from_dto(dto: ClientInfoDTO) -> StdResult<ClientInfo, FromDTOError> {
        Ok(ClientInfo {
            id: dto.id,
            secret: dto.secret,
            scopes: dto.scopes,
            request_limit: dto.request_limit,
        })
    }
}

/// Struct that holds all the profile information for the user.
#[derive(Clone, Debug)]
pub struct Profile {
    user_id: u64,
    display_name: String,
    first_name: Option<String>,
    last_name: Option<String>,
    image_url: Option<String>,
    age: Option<u8>,
    address: Option<String>,
    trust_score: i8,
}

impl Profile {
    /// Gets the user's ID.
    pub fn get_user_id(&self) -> u64 {
        self.user_id
    }

    /// Gets the display name of the user.
    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }

    /// Gets the first name of the user.
    pub fn get_first_name(&self) -> Option<&str> {
        match self.first_name.as_ref() {
            Some(n) => Some(n),
            None => None,
        }
    }

    /// Gets the last name of the user.
    pub fn get_last_name(&self) -> Option<&str> {
        match self.last_name.as_ref() {
            Some(n) => Some(n),
            None => None,
        }
    }

    /// Gets the image of the user.
    pub fn get_image_url(&self) -> Option<&str> {
        match self.image_url.as_ref() {
            Some(n) => Some(n),
            None => None,
        }
    }

    /// Gets the age of the user.
    pub fn get_age(&self) -> Option<u8> {
        self.age
    }

    /// Gets the address of the user.
    pub fn get_address(&self) -> Option<&str> {
        match self.address.as_ref() {
            Some(n) => Some(n),
            None => None,
        }
    }

    /// Gets the trust score of the user.
    pub fn get_trust_score(&self) -> i8 {
        self.trust_score
    }
}

#[cfg(feature = "json-types")]
impl json::ToJson for Profile {
    fn to_json(&self) -> json::Json {
        let mut object = json::Object::new();
        let _ = object.insert(String::from("user_id"), self.user_id.to_json());
        let _ = object.insert(String::from("display_name"), self.display_name.to_json());
        let _ = object.insert(String::from("first_name"), self.first_name.to_json());
        let _ = object.insert(String::from("last_name"), self.last_name.to_json());
        let _ = object.insert(String::from("image_url"), self.image_url.to_json());
        let _ = object.insert(String::from("age"), self.age.to_json());
        let _ = object.insert(String::from("address"), self.address.to_json());
        let _ = object.insert(String::from("trust_score"), self.trust_score.to_json());

        json::Json::Object(object)
    }
}

impl FromDTO<ProfileDTO> for Profile {
    fn from_dto(dto: ProfileDTO) -> StdResult<Profile, FromDTOError> {
        Ok(Profile {
            user_id: dto.user_id,
            display_name: dto.display_name,
            first_name: dto.first_name,
            last_name: dto.last_name,
            image_url: dto.image_url,
            age: dto.age,
            address: dto.address,
            trust_score: dto.trust_score,
        })
    }
}

/// Struct that holds all personal information for the user.
#[derive(Clone, Debug)]
pub struct User {
    /// The unique ID of the user
    user_id: u64,
    /// The unique username of the user
    username: String,
    /// Display name of the user
    display_name: String,
    /// The users email
    email: (String, bool),
    /// The users first name
    first_name: Option<(String, bool)>,
    /// The users last name
    last_name: Option<(String, bool)>,
    /// The amount of devices the user has
    device_count: u8,
    /// The list of users wallet addresses
    wallet_addresses: BTreeSet<WalletAddress>,
    /// The checking wallet balance
    checking_balance: Amount,
    /// The cold wallet balance
    cold_balance: Amount,
    /// The users pending balance
    pending_balance: Amount,
    /// the users date of birth
    birthday: Option<(NaiveDate, bool)>,
    /// the user's phone #
    phone: Option<(String, bool)>,
    /// The users profile images
    image_url: Option<String>,
    /// The users Address
    address: Option<(Address, bool)>,
    /// The users sybil score
    sybil_score: i8,
    /// The users trust score
    trust_score: i8,
    /// Whether the user account is enabled
    enabled: bool,
    /// The Date the user registered
    registration_time: DateTime<UTC>,
    /// The time the user was last seen doing an activity
    last_activity: DateTime<UTC>,
    /// Whether the user is banned
    banned: Option<DateTime<UTC>>,
}

impl User {
    /// Gets the ID of the user.
    pub fn get_id(&self) -> u64 {
        self.user_id
    }

    /// Gets the username of the user.
    pub fn get_username(&self) -> &str {
        &self.username
    }

    /// Gets the displayname of the user
    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }

    /// Gets the email of the user.
    pub fn get_email(&self) -> &str {
        &self.email.0
    }

    /// Returns wether the email of the user has been confirmed or not.
    pub fn is_email_confirmed(&self) -> bool {
        self.email.1
    }

    /// Gets the first name of the user, if it has been set.
    pub fn get_first_name(&self) -> Option<&str> {
        match self.first_name {
            Some((ref f, _c)) => Some(f),
            None => None,
        }
    }

    /// Returns wether the first name of the user has been confirmed or not.
    pub fn is_first_name_confirmed(&self) -> bool {
        match self.first_name {
            Some((_, c)) => c,
            None => false,
        }
    }

    /// Gets the last name of the user, if it has been set.
    pub fn get_last_name(&self) -> Option<&str> {
        match self.last_name {
            Some((ref l, _c)) => Some(l),
            None => None,
        }
    }

    /// Returns wether the last name of the user has been confirmed or not.
    pub fn is_last_name_confirmed(&self) -> bool {
        match self.last_name {
            Some((_, c)) => c,
            None => false,
        }
    }

    /// Gets the device count of the user.
    pub fn get_device_count(&self) -> u8 {
        self.device_count
    }

    /// Gets an iterator through the wallet addresses of the user.
    pub fn wallet_addresses(&self) -> btree_set::Iter<WalletAddress> {
        self.wallet_addresses.iter()
    }

    /// Gets the checking balance of the user.
    pub fn get_checking_balance(&self) -> Amount {
        self.checking_balance
    }

    /// Gets the cold balance of the user.
    pub fn get_cold_balance(&self) -> Amount {
        self.cold_balance
    }

    /// Gets the pending balance of the user.
    pub fn get_pending_balance(&self) -> Amount {
        self.pending_balance
    }

    /// Gets the birthday of the user, if it has been set.
    pub fn get_birthday(&self) -> Option<NaiveDate> {
        match self.birthday {
            Some((b, _c)) => Some(b),
            None => None,
        }
    }

    /// Returns wether the birthday of the user has been confirmed or not.
    pub fn is_birthday_confirmed(&self) -> bool {
        match self.birthday {
            Some((_b, c)) => c,
            None => false,
        }
    }

    /// Gets the phone of the user, if it has been set.
    pub fn get_phone(&self) -> Option<&str> {
        match self.phone {
            Some((ref p, _c)) => Some(p),
            None => None,
        }
    }

    /// Returns wether the phone of the user has been confirmed or not.
    pub fn is_phone_confirmed(&self) -> bool {
        match self.phone {
            Some((_, c)) => c,
            None => false,
        }
    }

    /// Gets the image of the user, if it has been set.
    pub fn get_image_url(&self) -> Option<&str> {
        match self.image_url {
            Some(ref i) => Some(i),
            None => None,
        }
    }

    /// Gets the address of the user, if it has been set.
    pub fn get_address(&self) -> Option<&Address> {
        match self.address {
            Some((ref a, _c)) => Some(a),
            None => None,
        }
    }

    /// Returns wether the address of the user is confirmed or not.
    pub fn is_address_confirmed(&self) -> bool {
        match self.address {
            Some((_, c)) => c,
            None => false,
        }
    }

    /// Gets the sybil score of the user.
    pub fn get_sybil_score(&self) -> i8 {
        self.sybil_score
    }

    /// Gets the trust score of the user.
    pub fn get_trust_score(&self) -> i8 {
        self.trust_score
    }

    /// Returns wether the user is enabled or not.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Gets the registration time of the user.
    pub fn get_registration_time(&self) -> DateTime<UTC> {
        self.registration_time
    }

    /// Gets the last activity time of the user.
    pub fn get_last_activity(&self) -> DateTime<UTC> {
        self.last_activity
    }

    /// Returns wether the user is banned or not.
    pub fn is_banned(&self) -> bool {
        self.banned.is_none() || self.banned.unwrap() > UTC::now()
    }

    /// Returns the ban's expiration time, if it has been set.
    pub fn ban_expiration(&self) -> Option<DateTime<UTC>> {
        self.banned
    }
}

#[cfg(feature = "json-types")]
impl json::ToJson for User {
    fn to_json(&self) -> json::Json {
        let mut object = json::Object::new();
        let _ = object.insert(String::from("user_id"), self.user_id.to_json());
        let _ = object.insert(String::from("username"), self.username.to_json());
        let _ = object.insert(String::from("display_name"), self.display_name.to_json());
        let _ = object.insert(String::from("email"), self.email.0.to_json());
        let _ = object.insert(String::from("email_confirmed"), self.email.1.to_json());
        match self.first_name {
            Some((ref f, c)) => {
                let _ = object.insert(String::from("first_name"), f.to_json());
                let _ = object.insert(String::from("first_name_confirmed"), c.to_json());
            }
            None => {
                let _ = object.insert(String::from("first_name"), None::<String>.to_json());
                let _ = object.insert(String::from("first_name_confirmed"), false.to_json());
            }
        }
        match self.last_name {
            Some((ref l, c)) => {
                let _ = object.insert(String::from("last_name"), l.to_json());
                let _ = object.insert(String::from("last_name_confirmed"), c.to_json());
            }
            None => {
                let _ = object.insert(String::from("last_name"), None::<String>.to_json());
                let _ = object.insert(String::from("last_name_confirmed"), false.to_json());
            }
        }
        let _ = object.insert(String::from("device_count"), self.device_count.to_json());
        let _ =
            object.insert(String::from("wallet_addresses"),
                          self.wallet_addresses.iter().map(|a| *a).collect::<Vec<_>>().to_json());
        let _ = object.insert(String::from("pending_balance"),
                              self.pending_balance.to_json());
        let _ = object.insert(String::from("checking_balance"),
                              self.checking_balance.to_json());
        let _ = object.insert(String::from("cold_balance"), self.cold_balance.to_json());
        match self.birthday {
            Some((ref b, c)) => {
                let _ = object.insert(String::from("birthday"), date_to_json(*b));
                let _ = object.insert(String::from("birthday_confirmed"), c.to_json());
            }
            None => {
                let _ = object.insert(String::from("birthday"), None::<String>.to_json());
                let _ = object.insert(String::from("birthday_confirmed"), false.to_json());
            }
        }
        match self.phone {
            Some((ref p, c)) => {
                let _ = object.insert(String::from("phone"), p.to_json());
                let _ = object.insert(String::from("phone_confirmed"), c.to_json());
            }
            None => {
                let _ = object.insert(String::from("phone"), None::<String>.to_json());
                let _ = object.insert(String::from("phone_confirmed"), false.to_json());
            }
        }
        let _ = object.insert(String::from("image_url"), self.image_url.to_json());
        match self.address {
            Some((ref a, c)) => {
                let _ = object.insert(String::from("address"), a.to_json());
                let _ = object.insert(String::from("address_confirmed"), c.to_json());
            }
            None => {
                let _ = object.insert(String::from("address"), None::<String>.to_json());
                let _ = object.insert(String::from("address_confirmed"), false.to_json());
            }
        }
        let _ = object.insert(String::from("sybil_score"), self.sybil_score.to_json());
        let _ = object.insert(String::from("trust_score"), self.trust_score.to_json());
        let _ = object.insert(String::from("enabled"), self.enabled.to_json());
        let _ = object.insert(String::from("registration_time"),
                              time_to_json(self.registration_time));
        let _ = object.insert(String::from("last_activity"),
                              time_to_json(self.last_activity));
        let _ = object.insert(String::from("is_banned"), self.is_banned().to_json());
        match self.banned {
            Some(d) => {
                let _ = object.insert(String::from("ban_end"), time_to_json(d));
            }
            None => {
                let _ = object.insert(String::from("ban_end"), None::<String>.to_json());
            }
        }


        json::Json::Object(object)
    }
}

impl FromDTO<UserDTO> for User {
    fn from_dto(dto: UserDTO) -> StdResult<User, FromDTOError> {

        let first_opt = match dto.first_name {
            Some(first) => Some((first, dto.first_name_confirmed)),
            None => None,
        };

        let last_opt = match dto.last_name {
            Some(last) => Some((last, dto.last_name_confirmed)),
            None => None,
        };

        let birthday_opt = match dto.birthday {
            Some(birthday) => Some((birthday, dto.birthday_confirmed)),
            None => None,
        };

        let phone_opt = match dto.phone {
            Some(phone) => Some((phone, dto.phone_confirmed)),
            None => None,
        };

        let adress_opt = match dto.address {
            Some(address) => Some((address, dto.address_confirmed)),
            None => None,
        };

        Ok(User {
            user_id: dto.user_id,
            username: dto.username,
            display_name: dto.display_name,
            email: (dto.email, dto.email_confirmed),
            first_name: first_opt,
            last_name: last_opt,
            device_count: dto.device_count,
            pending_balance: dto.pending_balance,
            wallet_addresses: dto.wallet_addresses,
            checking_balance: dto.checking_balance,
            cold_balance: dto.cold_balance,
            birthday: birthday_opt,
            phone: phone_opt,
            image_url: dto.image_url,
            address: adress_opt,
            sybil_score: dto.sybil_score,
            trust_score: dto.trust_score,
            enabled: dto.enabled,
            registration_time: dto.registration_time,
            last_activity: dto.last_activity,
            banned: dto.banned,
        })
    }
}

/// The representation of a global credit transaction
#[derive(Clone, Debug)]
pub struct Transaction {
    /// The id of the transaction
    transaction_id: u64,
    /// The origin of the transaction
    origin_user: Profile,
    /// The destination of the transaction
    destination_user: Profile,
    /// The destination address of the transaction
    destination_address: WalletAddress,
    /// The amount of the transaction
    amount: Amount,
    /// The timestamp of the transaction
    timestamp: DateTime<UTC>,
}

impl Transaction {
    /// Returns the id of the transaction
    pub fn get_id(&self) -> u64 {
        self.transaction_id
    }
    /// Retruns the id of the user receiving the transaction
    pub fn get_destination_user(&self) -> &Profile {
        &self.destination_user
    }
    /// Returns the wallet address receiving the transaction
    pub fn get_destination_address(&self) -> &WalletAddress {
        &self.destination_address
    }
    /// Returns the user id sending the transaction
    pub fn get_origin_user(&self) -> &Profile {
        &self.origin_user
    }
    /// The amount of the transaction in global credits
    pub fn get_amount(&self) -> Amount {
        self.amount
    }
    /// The timestamp of the transaction
    pub fn get_timestamp(&self) -> &DateTime<UTC> {
        &self.timestamp
    }
}

#[cfg(feature = "json-types")]
impl json::ToJson for Transaction {
    fn to_json(&self) -> json::Json {
        let mut object = json::Object::new();
        let _ = object.insert(String::from("transaction_id"),
                              self.transaction_id.to_json());
        let _ = object.insert(String::from("origin_user"), self.origin_user.to_json());
        let _ = object.insert(String::from("destination_user"),
                              self.destination_user.to_json());
        let _ = object.insert(String::from("destination_address"),
                              self.destination_address.to_json());
        let _ = object.insert(String::from("amount"), self.amount.to_json());
        let _ = object.insert(String::from("timestamp"), time_to_json(self.timestamp));
        json::Json::Object(object)
    }
}

impl FromDTO<TransactionDTO> for Transaction {
    fn from_dto(dto: TransactionDTO) -> StdResult<Transaction, FromDTOError> {
        Ok(Transaction {
            transaction_id: dto.transaction_id,
            origin_user: Profile::from_dto(dto.origin_user)?,
            destination_user: Profile::from_dto(dto.destination_user)?,
            destination_address: dto.destination_address,
            amount: dto.amount,
            timestamp: dto.timestamp,
        })
    }
}

/// Pending friend request.
#[derive(Clone, Debug)]
pub struct PendingFriendRequest {
    /// Connection ID.
    connection_id: u64,
    /// Origin user's profile.
    origin_user: Profile,
    /// Destination user's profile.
    destination_user: Profile,
    /// Request message.
    message: Option<String>,
}

impl PendingFriendRequest {
    /// Gets the connection ID of the friend request.
    pub fn get_connection_id(&self) -> u64 {
        self.connection_id
    }

    /// Gets the origin user's profile of the friend request.
    pub fn get_origin_user(&self) -> &Profile {
        &self.origin_user
    }

    /// Gets the destination user's profile of the friend request.
    pub fn get_destination_user(&self) -> &Profile {
        &self.destination_user
    }

    /// Gets the message of the friend request.
    pub fn get_message(&self) -> Option<&str> {
        match self.message.as_ref() {
            Some(m) => Some(m),
            None => None,
        }
    }
}

#[cfg(feature = "json-types")]
impl json::ToJson for PendingFriendRequest {
    fn to_json(&self) -> json::Json {
        let mut object = json::Object::new();
        let _ = object.insert(String::from("connection_id"), self.connection_id.to_json());
        let _ = object.insert(String::from("origin_user"), self.origin_user.to_json());
        let _ = object.insert(String::from("destination_user"),
                              self.destination_user.to_json());
        let _ = object.insert(String::from("message"), self.message.to_json());
        json::Json::Object(object)
    }
}

impl FromDTO<PendingFriendRequestDTO> for PendingFriendRequest {
    fn from_dto(dto: PendingFriendRequestDTO) -> StdResult<PendingFriendRequest, FromDTOError> {
        Ok(PendingFriendRequest {
            connection_id: dto.connection_id,
            origin_user: Profile::from_dto(dto.origin_user)?,
            destination_user: Profile::from_dto(dto.destination_user)?,
            message: dto.message,
        })
    }
}

#[cfg(feature = "json-types")]
fn time_to_json(time: DateTime<UTC>) -> json::Json {
    use chrono::{Timelike, Datelike};
    use rustc_serialize::json::ToJson;

    let mut object = json::Object::new();
    let _ = object.insert(String::from("day"), time.day().to_json());
    let _ = object.insert(String::from("mon"), time.month().to_json());
    let _ = object.insert(String::from("year"), time.year().to_json());

    let _ = object.insert(String::from("hour"), time.hour().to_json());
    let _ = object.insert(String::from("min"), time.minute().to_json());
    let _ = object.insert(String::from("sec"), time.second().to_json());
    json::Json::Object(object)
}

#[cfg(feature = "json-types")]
fn date_to_json(time: NaiveDate) -> json::Json {
    use chrono::Datelike;
    use rustc_serialize::json::ToJson;

    let mut object = json::Object::new();
    let _ = object.insert(String::from("day"), time.day().to_json());
    let _ = object.insert(String::from("mon"), time.month().to_json());
    let _ = object.insert(String::from("year"), time.year().to_json());
    json::Json::Object(object)
}
