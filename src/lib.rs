use std::fs;
use std::path::PathBuf;
use indexmap::{IndexMap, indexmap};
use tree_sitter_tags::{TagsConfiguration, TagsContext};

// Open source file
#[allow(dead_code)]
fn read_fixture(fixture_name: &str) -> Vec<u8> {
    let file_path = fs::canonicalize(PathBuf::from(format!("./fixtures/{}", fixture_name)));
    fs::read(file_path.unwrap()).expect("Can't open fixture file")
}

// Parse tags from supplied source.
#[allow(dead_code)]
fn parse_tags(config: TagsConfiguration, source: &[u8]) -> Vec<IndexMap<&str, String>> {
    let mut context = TagsContext::new();

    let tags = context.generate_tags(&config, source, None).expect("Can't parse source code").0;

    let mut parsed: Vec<IndexMap<&str, String>> = vec![];
    for result in tags {
        let tag = result.unwrap();

        // SEE: https://github.com/tree-sitter/tree-sitter/blob/v0.20.4/cli/src/tags.rs#L64-L71
        let name = std::str::from_utf8(&source[tag.name_range]).unwrap_or("");
        let kind = config.syntax_type_name(tag.syntax_type_id);
        let def_or_ref = if tag.is_definition { "def" } else { "ref" };
        let first_line = std::str::from_utf8(&source[tag.line_range]).unwrap_or("");

        parsed.push(indexmap! {
                // Wrap all variables as String for preventing exposing local variable reference that Rust compiler would complain :(
                "name" => String::from(name),
                "kind" => String::from(kind),
                "def_or_ref" => String::from(def_or_ref),
                "first_line" => String::from(first_line),
            });
    }

    parsed
}

#[allow(dead_code)]
fn parse_js(source: &[u8]) -> Vec<IndexMap<&str, String>> {
    let config = TagsConfiguration::new(
        tree_sitter_javascript::language(),
        tree_sitter_javascript::TAGGING_QUERY,
        tree_sitter_javascript::LOCALS_QUERY,
    ).unwrap();
    parse_tags(config, source)
}

#[allow(dead_code)]
fn parse_ts(source: &[u8]) -> Vec<IndexMap<&str, String>> {
    // SEE: https://github.com/tree-sitter/tree-sitter-typescript/blob/v0.20.1/package.json#L45-L52
    let tags_query = tree_sitter_typescript::TAGGING_QUERY.to_owned() + tree_sitter_javascript::TAGGING_QUERY;
    let locals_query = tree_sitter_typescript::LOCALS_QUERY.to_owned() + tree_sitter_javascript::LOCALS_QUERY;
    let config = TagsConfiguration::new(
        tree_sitter_typescript::language_typescript(),
        &tags_query,
        &locals_query,
    ).unwrap();
    parse_tags(config, source)
}

#[allow(dead_code)]
fn parse_rb(source: &[u8]) -> Vec<IndexMap<&str, String>> {
    let config = TagsConfiguration::new(
        tree_sitter_ruby::language(),
        tree_sitter_ruby::TAGGING_QUERY,
        tree_sitter_ruby::LOCALS_QUERY,
    ).unwrap();
    parse_tags(config, source)
}

#[allow(dead_code)]
fn parse_php(source: &[u8]) -> Vec<IndexMap<&str, String>> {
    let config = TagsConfiguration::new(
        tree_sitter_php::language(),
        tree_sitter_php::TAGS_QUERY,
        "",
    ).unwrap();
    parse_tags(config, source)
}

#[allow(dead_code)]
fn parse_py(source: &[u8]) -> Vec<IndexMap<&str, String>> {
    let config = TagsConfiguration::new(
        tree_sitter_python::language(),
        tree_sitter_python::TAGGING_QUERY,
        "",
    ).unwrap();
    parse_tags(config, source)
}

#[cfg(test)]
mod tests {
    use indexmap::{indexmap, IndexMap};
    use super::*;

    #[test]
    fn it_should_allow_js() {
        let source = read_fixture("Animal.js");
        let tags = parse_js(&source);

        let expected: Vec<IndexMap<&str, &str>> = vec![
            indexmap! {"name" => "Animal", "kind" => "class", "def_or_ref" => "def", "first_line" => "class Animal extends Model {"},
            indexmap! {"name" => "tableName", "kind" => "method", "def_or_ref" => "def", "first_line" => "static get tableName() {"},
            indexmap! {"name" => "jsonSchema", "kind" => "method", "def_or_ref" => "def", "first_line" => "static get jsonSchema() {"},
            indexmap! {"name" => "relationMappings", "kind" => "method", "def_or_ref" => "def", "first_line" => "static get relationMappings() {"},
        ];

        assert_eq!(tags, expected);
    }

    #[test]
    fn it_should_allow_ts() {
        let source = read_fixture("Post.ts");
        let tags = parse_ts(&source);

        let expected: Vec<IndexMap<&str, &str>> = vec![
            indexmap! {"name" => "Entity", "kind" => "call", "def_or_ref" => "ref", "first_line" => "@Entity(\"sample10_post\")"},
            indexmap! {"name" => "Post", "kind" => "class", "def_or_ref" => "def", "first_line" => "export class Post {"},
            indexmap! {"name" => "PrimaryGeneratedColumn", "kind" => "call", "def_or_ref" => "ref", "first_line" => "@PrimaryGeneratedColumn()"},
            indexmap! {"name" => "Column", "kind" => "call", "def_or_ref" => "ref", "first_line" => "@Column({"},
            indexmap! {"name" => "Column", "kind" => "call", "def_or_ref" => "ref", "first_line" => "@Column({"},
            indexmap! {"name" => "OneToOne", "kind" => "call", "def_or_ref" => "ref", "first_line" => "@OneToOne((type) => PostDetails, (details) => details.post, {"},
            indexmap! {"name" => "JoinColumn", "kind" => "call", "def_or_ref" => "ref", "first_line" => "@JoinColumn()"},
            indexmap! {"name" => "PostDetails", "kind" => "type", "def_or_ref" => "ref", "first_line" => "details: PostDetails"},
            indexmap! {"name" => "OneToMany", "kind" => "call", "def_or_ref" => "ref", "first_line" => "@OneToMany((type) => Image, (image) => image.post, {"},
            indexmap! {"name" => "OneToMany", "kind" => "call", "def_or_ref" => "ref", "first_line" => "@OneToMany((type) => Image, (image) => image.secondaryPost)"},
            indexmap! {"name" => "ManyToOne", "kind" => "call", "def_or_ref" => "ref", "first_line" => "@ManyToOne((type) => Cover, (cover) => cover.posts, {"},
            indexmap! {"name" => "JoinColumn", "kind" => "call", "def_or_ref" => "ref", "first_line" => "@JoinColumn({ name: \"coverId\" })"},
            indexmap! {"name" => "Cover", "kind" => "type", "def_or_ref" => "ref", "first_line" => "cover: Cover"},
            indexmap! {"name" => "Column", "kind" => "call", "def_or_ref" => "ref", "first_line" => "@Column(\"int\", {"},
            indexmap! {"name" => "ManyToMany", "kind" => "call", "def_or_ref" => "ref", "first_line" => "@ManyToMany((type) => Category, (category) => category.posts, {"},
            indexmap! {"name" => "JoinTable", "kind" => "call", "def_or_ref" => "ref", "first_line" => "@JoinTable()"}
        ];

        assert_eq!(tags, expected);
    }

    #[test]
    fn it_should_allow_rb() {
        let source = read_fixture("user.rb");
        let tags = parse_rb(&source);

        let expected: Vec<IndexMap<&str, &str>> = vec![
            indexmap! {"name" => "User", "kind" => "class", "def_or_ref" => "def", "first_line" => "class User < ApplicationRecord"},
            indexmap! {"name" => "ApplicationRecord", "kind" => "call", "def_or_ref" => "ref", "first_line" => "class User < ApplicationRecord"},
            indexmap! {"name" => "has_many", "kind" => "call", "def_or_ref" => "ref", "first_line" => "has_many :microposts, dependent: :destroy"},
            indexmap! {"name" => "has_many", "kind" => "call", "def_or_ref" => "ref", "first_line" => "has_many :active_relationships, class_name:  \"Relationship\","},
            indexmap! {"name" => "has_many", "kind" => "call", "def_or_ref" => "ref", "first_line" => "has_many :passive_relationships, class_name:  \"Relationship\","},
            indexmap! {"name" => "has_many", "kind" => "call", "def_or_ref" => "ref", "first_line" => "has_many :following, through: :active_relationships,  source: :followed"},
            indexmap! {"name" => "has_many", "kind" => "call", "def_or_ref" => "ref", "first_line" => "has_many :followers, through: :passive_relationships, source: :follower"},
            indexmap! {"name" => "attr_accessor", "kind" => "call", "def_or_ref" => "ref", "first_line" => "attr_accessor :remember_token, :activation_token, :reset_token"},
            indexmap! {"name" => "before_save", "kind" => "call", "def_or_ref" => "ref", "first_line" => "before_save   :downcase_email"},
            indexmap! {"name" => "before_create", "kind" => "call", "def_or_ref" => "ref", "first_line" => "before_create :create_activation_digest"},
            indexmap! {"name" => "validates", "kind" => "call", "def_or_ref" => "ref", "first_line" => "validates :name,  presence: true, length: { maximum: 50 }"},
            indexmap! {"name" => "VALID_EMAIL_REGEX", "kind" => "call", "def_or_ref" => "ref", "first_line" => "VALID_EMAIL_REGEX = /\\A[\\w+\\-.]+@[a-z\\d\\-.]+\\.[a-z]+\\z/i"},
            indexmap! {"name" => "validates", "kind" => "call", "def_or_ref" => "ref", "first_line" => "validates :email, presence: true, length: { maximum: 255 },"},
            indexmap! {"name" => "VALID_EMAIL_REGEX", "kind" => "call", "def_or_ref" => "ref", "first_line" => "format: { with: VALID_EMAIL_REGEX },"},
            indexmap! {"name" => "has_secure_password", "kind" => "call", "def_or_ref" => "ref", "first_line" => "has_secure_password"},
            indexmap! {"name" => "validates", "kind" => "call", "def_or_ref" => "ref", "first_line" => "validates :password, presence: true, length: { minimum: 6 }, allow_nil: true"},
            indexmap! {"name" => "User", "kind" => "call", "def_or_ref" => "ref", "first_line" => "def User.digest(string)"},
            indexmap! {"name" => "digest", "kind" => "method", "def_or_ref" => "def", "first_line" => "def User.digest(string)"},
            indexmap! {"name" => "ActiveModel", "kind" => "call", "def_or_ref" => "ref", "first_line" => "cost = ActiveModel::SecurePassword.min_cost ? BCrypt::Engine::MIN_COST :"},
            indexmap! {"name" => "SecurePassword", "kind" => "call", "def_or_ref" => "ref", "first_line" => "cost = ActiveModel::SecurePassword.min_cost ? BCrypt::Engine::MIN_COST :"},
            indexmap! {"name" => "min_cost", "kind" => "call", "def_or_ref" => "ref", "first_line" => "cost = ActiveModel::SecurePassword.min_cost ? BCrypt::Engine::MIN_COST :"},
            indexmap! {"name" => "BCrypt", "kind" => "call", "def_or_ref" => "ref", "first_line" => "cost = ActiveModel::SecurePassword.min_cost ? BCrypt::Engine::MIN_COST :"},
            indexmap! {"name" => "Engine", "kind" => "call", "def_or_ref" => "ref", "first_line" => "cost = ActiveModel::SecurePassword.min_cost ? BCrypt::Engine::MIN_COST :"},
            indexmap! {"name" => "MIN_COST", "kind" => "call", "def_or_ref" => "ref", "first_line" => "cost = ActiveModel::SecurePassword.min_cost ? BCrypt::Engine::MIN_COST :"},
            indexmap! {"name" => "BCrypt", "kind" => "call", "def_or_ref" => "ref", "first_line" => "BCrypt::Engine.cost"},
            indexmap! {"name" => "Engine", "kind" => "call", "def_or_ref" => "ref", "first_line" => "BCrypt::Engine.cost"},
            indexmap! {"name" => "cost", "kind" => "call", "def_or_ref" => "ref", "first_line" => "BCrypt::Engine.cost"},
            indexmap! {"name" => "BCrypt", "kind" => "call", "def_or_ref" => "ref", "first_line" => "BCrypt::Password.create(string, cost: cost)"},
            indexmap! {"name" => "Password", "kind" => "call", "def_or_ref" => "ref", "first_line" => "BCrypt::Password.create(string, cost: cost)"},
            indexmap! {"name" => "create", "kind" => "call", "def_or_ref" => "ref", "first_line" => "BCrypt::Password.create(string, cost: cost)"},
            indexmap! {"name" => "User", "kind" => "call", "def_or_ref" => "ref", "first_line" => "def User.new_token"},
            indexmap! {"name" => "new_token", "kind" => "method", "def_or_ref" => "def", "first_line" => "def User.new_token"},
            indexmap! {"name" => "SecureRandom", "kind" => "call", "def_or_ref" => "ref", "first_line" => "SecureRandom.urlsafe_base64"},
            indexmap! {"name" => "urlsafe_base64", "kind" => "call", "def_or_ref" => "ref", "first_line" => "SecureRandom.urlsafe_base64"},
            indexmap! {"name" => "remember", "kind" => "method", "def_or_ref" => "def", "first_line" => "def remember"},
            indexmap! {"name" => "remember_token", "kind" => "call", "def_or_ref" => "ref", "first_line" => "self.remember_token = User.new_token"},
            indexmap! {"name" => "User", "kind" => "call", "def_or_ref" => "ref", "first_line" => "self.remember_token = User.new_token"},
            indexmap! {"name" => "new_token", "kind" => "call", "def_or_ref" => "ref", "first_line" => "self.remember_token = User.new_token"},
            indexmap! {"name" => "update_attribute", "kind" => "call", "def_or_ref" => "ref", "first_line" => "update_attribute(:remember_digest, User.digest(remember_token))"},
            indexmap! {"name" => "User", "kind" => "call", "def_or_ref" => "ref", "first_line" => "update_attribute(:remember_digest, User.digest(remember_token))"},
            indexmap! {"name" => "digest", "kind" => "call", "def_or_ref" => "ref", "first_line" => "update_attribute(:remember_digest, User.digest(remember_token))"},
            indexmap! {"name" => "remember_token", "kind" => "call", "def_or_ref" => "ref", "first_line" => "update_attribute(:remember_digest, User.digest(remember_token))"},
            indexmap! {"name" => "remember_digest", "kind" => "call", "def_or_ref" => "ref", "first_line" => "remember_digest"},
            indexmap! {"name" => "session_token", "kind" => "method", "def_or_ref" => "def", "first_line" => "def session_token"},
            indexmap! {"name" => "remember_digest", "kind" => "call", "def_or_ref" => "ref", "first_line" => "remember_digest || remember"},
            indexmap! {"name" => "remember", "kind" => "call", "def_or_ref" => "ref", "first_line" => "remember_digest || remember"},
            indexmap! {"name" => "authenticated?", "kind" => "method", "def_or_ref" => "def", "first_line" => "def authenticated?(attribute, token)"},
            indexmap! {"name" => "send", "kind" => "call", "def_or_ref" => "ref", "first_line" => "digest = send(\"#{attribute}_digest\")"},
            indexmap! {"name" => "nil?", "kind" => "call", "def_or_ref" => "ref", "first_line" => "return false if digest.nil?"},
            indexmap! {"name" => "BCrypt", "kind" => "call", "def_or_ref" => "ref", "first_line" => "BCrypt::Password.new(digest).is_password?(token)"},
            indexmap! {"name" => "Password", "kind" => "call", "def_or_ref" => "ref", "first_line" => "BCrypt::Password.new(digest).is_password?(token)"},
            indexmap! {"name" => "new", "kind" => "call", "def_or_ref" => "ref", "first_line" => "BCrypt::Password.new(digest).is_password?(token)"},
            indexmap! {"name" => "is_password?", "kind" => "call", "def_or_ref" => "ref", "first_line" => "BCrypt::Password.new(digest).is_password?(token)"},
            indexmap! {"name" => "forget", "kind" => "method", "def_or_ref" => "def", "first_line" => "def forget"},
            indexmap! {"name" => "update_attribute", "kind" => "call", "def_or_ref" => "ref", "first_line" => "update_attribute(:remember_digest, nil)"},
            indexmap! {"name" => "activate", "kind" => "method", "def_or_ref" => "def", "first_line" => "def activate"},
            indexmap! {"name" => "update_attribute", "kind" => "call", "def_or_ref" => "ref", "first_line" => "update_attribute(:activated,    true)"},
            indexmap! {"name" => "update_attribute", "kind" => "call", "def_or_ref" => "ref", "first_line" => "update_attribute(:activated_at, Time.zone.now)"},
            indexmap! {"name" => "Time", "kind" => "call", "def_or_ref" => "ref", "first_line" => "update_attribute(:activated_at, Time.zone.now)"},
            indexmap! {"name" => "zone", "kind" => "call", "def_or_ref" => "ref", "first_line" => "update_attribute(:activated_at, Time.zone.now)"},
            indexmap! {"name" => "now", "kind" => "call", "def_or_ref" => "ref", "first_line" => "update_attribute(:activated_at, Time.zone.now)"},
            indexmap! {"name" => "send_activation_email", "kind" => "method", "def_or_ref" => "def", "first_line" => "def send_activation_email"},
            indexmap! {"name" => "UserMailer", "kind" => "call", "def_or_ref" => "ref", "first_line" => "UserMailer.account_activation(self).deliver_now"},
            indexmap! {"name" => "account_activation", "kind" => "call", "def_or_ref" => "ref", "first_line" => "UserMailer.account_activation(self).deliver_now"},
            indexmap! {"name" => "deliver_now", "kind" => "call", "def_or_ref" => "ref", "first_line" => "UserMailer.account_activation(self).deliver_now"},
            indexmap! {"name" => "create_reset_digest", "kind" => "method", "def_or_ref" => "def", "first_line" => "def create_reset_digest"},
            indexmap! {"name" => "reset_token", "kind" => "call", "def_or_ref" => "ref", "first_line" => "self.reset_token = User.new_token"},
            indexmap! {"name" => "User", "kind" => "call", "def_or_ref" => "ref", "first_line" => "self.reset_token = User.new_token"},
            indexmap! {"name" => "new_token", "kind" => "call", "def_or_ref" => "ref", "first_line" => "self.reset_token = User.new_token"},
            indexmap! {"name" => "update_attribute", "kind" => "call", "def_or_ref" => "ref", "first_line" => "update_attribute(:reset_digest,  User.digest(reset_token))"},
            indexmap! {"name" => "User", "kind" => "call", "def_or_ref" => "ref", "first_line" => "update_attribute(:reset_digest,  User.digest(reset_token))"},
            indexmap! {"name" => "digest", "kind" => "call", "def_or_ref" => "ref", "first_line" => "update_attribute(:reset_digest,  User.digest(reset_token))"},
            indexmap! {"name" => "reset_token", "kind" => "call", "def_or_ref" => "ref", "first_line" => "update_attribute(:reset_digest,  User.digest(reset_token))"},
            indexmap! {"name" => "update_attribute", "kind" => "call", "def_or_ref" => "ref", "first_line" => "update_attribute(:reset_sent_at, Time.zone.now)"},
            indexmap! {"name" => "Time", "kind" => "call", "def_or_ref" => "ref", "first_line" => "update_attribute(:reset_sent_at, Time.zone.now)"},
            indexmap! {"name" => "zone", "kind" => "call", "def_or_ref" => "ref", "first_line" => "update_attribute(:reset_sent_at, Time.zone.now)"},
            indexmap! {"name" => "now", "kind" => "call", "def_or_ref" => "ref", "first_line" => "update_attribute(:reset_sent_at, Time.zone.now)"},
            indexmap! {"name" => "send_password_reset_email", "kind" => "method", "def_or_ref" => "def", "first_line" => "def send_password_reset_email"},
            indexmap! {"name" => "UserMailer", "kind" => "call", "def_or_ref" => "ref", "first_line" => "UserMailer.password_reset(self).deliver_now"},
            indexmap! {"name" => "password_reset", "kind" => "call", "def_or_ref" => "ref", "first_line" => "UserMailer.password_reset(self).deliver_now"},
            indexmap! {"name" => "deliver_now", "kind" => "call", "def_or_ref" => "ref", "first_line" => "UserMailer.password_reset(self).deliver_now"},
            indexmap! {"name" => "password_reset_expired?", "kind" => "method", "def_or_ref" => "def", "first_line" => "def password_reset_expired?"},
            indexmap! {"name" => "reset_sent_at", "kind" => "call", "def_or_ref" => "ref", "first_line" => "reset_sent_at < 2.hours.ago"},
            indexmap! {"name" => "hours", "kind" => "call", "def_or_ref" => "ref", "first_line" => "reset_sent_at < 2.hours.ago"},
            indexmap! {"name" => "ago", "kind" => "call", "def_or_ref" => "ref", "first_line" => "reset_sent_at < 2.hours.ago"},
            indexmap! {"name" => "feed", "kind" => "method", "def_or_ref" => "def", "first_line" => "def feed"},
            indexmap! {"name" => "Micropost", "kind" => "call", "def_or_ref" => "ref", "first_line" => "Micropost.where(\"user_id IN (#{following_ids})"},
            indexmap! {"name" => "where", "kind" => "call", "def_or_ref" => "ref", "first_line" => "Micropost.where(\"user_id IN (#{following_ids})"},
            indexmap! {"name" => "id", "kind" => "call", "def_or_ref" => "ref", "first_line" => "OR user_id = :user_id\", user_id: id)"},
            indexmap! {"name" => "includes", "kind" => "call", "def_or_ref" => "ref", "first_line" => ".includes(:user, image_attachment: :blob)"},
            indexmap! {"name" => "follow", "kind" => "method", "def_or_ref" => "def", "first_line" => "def follow(other_user)"},
            indexmap! {"name" => "following", "kind" => "call", "def_or_ref" => "ref", "first_line" => "following << other_user unless self == other_user"},
            indexmap! {"name" => "unfollow", "kind" => "method", "def_or_ref" => "def", "first_line" => "def unfollow(other_user)"},
            indexmap! {"name" => "following", "kind" => "call", "def_or_ref" => "ref", "first_line" => "following.delete(other_user)"},
            indexmap! {"name" => "delete", "kind" => "call", "def_or_ref" => "ref", "first_line" => "following.delete(other_user)"},
            indexmap! {"name" => "following?", "kind" => "method", "def_or_ref" => "def", "first_line" => "def following?(other_user)"},
            indexmap! {"name" => "following", "kind" => "call", "def_or_ref" => "ref", "first_line" => "following.include?(other_user)"},
            indexmap! {"name" => "include?", "kind" => "call", "def_or_ref" => "ref", "first_line" => "following.include?(other_user)"},
            indexmap! {"name" => "private", "kind" => "call", "def_or_ref" => "ref", "first_line" => "private"},
            indexmap! {"name" => "downcase_email", "kind" => "method", "def_or_ref" => "def", "first_line" => "def downcase_email"},
            indexmap! {"name" => "email", "kind" => "call", "def_or_ref" => "ref", "first_line" => "self.email = email.downcase"},
            indexmap! {"name" => "email", "kind" => "call", "def_or_ref" => "ref", "first_line" => "self.email = email.downcase"},
            indexmap! {"name" => "downcase", "kind" => "call", "def_or_ref" => "ref", "first_line" => "self.email = email.downcase"},
            indexmap! {"name" => "create_activation_digest", "kind" => "method", "def_or_ref" => "def", "first_line" => "def create_activation_digest"},
            indexmap! {"name" => "activation_token", "kind" => "call", "def_or_ref" => "ref", "first_line" => "self.activation_token  = User.new_token"},
            indexmap! {"name" => "User", "kind" => "call", "def_or_ref" => "ref", "first_line" => "self.activation_token  = User.new_token"},
            indexmap! {"name" => "new_token", "kind" => "call", "def_or_ref" => "ref", "first_line" => "self.activation_token  = User.new_token"},
            indexmap! {"name" => "activation_digest", "kind" => "call", "def_or_ref" => "ref", "first_line" => "self.activation_digest = User.digest(activation_token)"},
            indexmap! {"name" => "User", "kind" => "call", "def_or_ref" => "ref", "first_line" => "self.activation_digest = User.digest(activation_token)"},
            indexmap! {"name" => "digest", "kind" => "call", "def_or_ref" => "ref", "first_line" => "self.activation_digest = User.digest(activation_token)"},
            indexmap! {"name" => "activation_token", "kind" => "call", "def_or_ref" => "ref", "first_line" => "self.activation_digest = User.digest(activation_token)"}
        ];

        assert_eq!(tags, expected);
    }

    #[test]
    fn it_should_allow_php() {
        let source = read_fixture("User.php");
        let tags = parse_php(&source);

        let expected: Vec<IndexMap<&str, &str>> = vec![
            indexmap! {"name" => "User", "kind" => "class", "def_or_ref" => "def", "first_line" => "class User extends Authenticatable"},
            indexmap! {"name" => "setPasswordAttribute", "kind" => "function", "def_or_ref" => "def", "first_line" => "public function setPasswordAttribute($password)"},
            indexmap! {"name" => "posts", "kind" => "function", "def_or_ref" => "def", "first_line" => "public function posts()"},
            indexmap! {"name" => "hasMany", "kind" => "call", "def_or_ref" => "ref", "first_line" => "return $this->hasMany(Post::class);"}
        ];

        assert_eq!(tags, expected);
    }

    #[test]
    fn it_should_allow_py() {
        let source = read_fixture("models.py");
        let tags = parse_py(&source);

        let expected: Vec<IndexMap<&str, &str>> = vec![
            indexmap! {"name" => "Question", "kind" => "class", "def_or_ref" => "def", "first_line" => "class Question(models.Model):"},
            indexmap! {"name" => "CharField", "kind" => "call", "def_or_ref" => "ref", "first_line" => "question_text = models.CharField(max_length=200)"},
            indexmap! {"name" => "DateTimeField", "kind" => "call", "def_or_ref" => "ref", "first_line" => "pub_date = models.DateTimeField('date published')"},
            indexmap! {"name" => "__str__", "kind" => "function", "def_or_ref" => "def", "first_line" => "def __str__(self):"},
            indexmap! {"name" => "was_published_recently", "kind" => "function", "def_or_ref" => "def", "first_line" => "def was_published_recently(self):"},
            indexmap! {"name" => "now", "kind" => "call", "def_or_ref" => "ref", "first_line" => "now = timezone.now()"},
            indexmap! {"name" => "timedelta", "kind" => "call", "def_or_ref" => "ref", "first_line" => "return now - datetime.timedelta(days=1) <= self.pub_date <= now"},
            indexmap! {"name" => "Choice", "kind" => "class", "def_or_ref" => "def", "first_line" => "class Choice(models.Model):"},
            indexmap! {"name" => "ForeignKey", "kind" => "call", "def_or_ref" => "ref", "first_line" => "question = models.ForeignKey(Question, on_delete=models.CASCADE)"},
            indexmap! {"name" => "CharField", "kind" => "call", "def_or_ref" => "ref", "first_line" => "choice_text = models.CharField(max_length=200)"},
            indexmap! {"name" => "IntegerField", "kind" => "call", "def_or_ref" => "ref", "first_line" => "votes = models.IntegerField(default=0)"},
            indexmap! {"name" => "__str__", "kind" => "function", "def_or_ref" => "def", "first_line" => "def __str__(self):"}
        ];

        assert_eq!(tags, expected);
    }
}