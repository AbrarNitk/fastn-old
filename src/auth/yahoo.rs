#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UserDetail {
    pub token: String,
    pub user_name: String,
    pub user_id: String,
}
pub async fn matched_identities(
    _ud: UserDetail,
    _identities: &[fpm::user_group::UserIdentity],
) -> fpm::Result<Vec<fpm::user_group::UserIdentity>> {
    /*let yahoo_identities = identities
        .iter()
        .filter(|identity| identity.key.starts_with("yahoo"))
        .collect::<Vec<&fpm::user_group::UserIdentity>>();

    if yahoo_identities.is_empty() {
        return Ok(vec![]);
    }*/

    let matched_identities = vec![];

    Ok(matched_identities)
}