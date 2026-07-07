use git2okf::git::repository::get_repository_name;

#[test]
fn test_repository_name_extraction() {
    let url = "https://github.com/laravel/framework";
    let name = get_repository_name(url);
    assert_eq!(name, "laravel/framework");

    let url2 = "git@github.com:facebook/react.git";
    let name2 = get_repository_name(url2);
    assert_eq!(name2, "facebook/react");
}
