use models::picture::Picture;
use maud::PreEscaped;

pub fn show(picture: Picture) -> PreEscaped<String> {
    let mut body = String::new();

    html!(body, {
        article.picture id=^(format!("picture_{}", picture.id)) {


                div.picture__title {
                    span "title:"
                    span ^(picture.title)
                }


            ul.picture__actions.actions {
                li a href=^(format!("/picture/{}/edit", picture.id)) "Edit Picture"
                li a href="/pictures" "View All"
            }
        }
    }).unwrap();

    PreEscaped(body)
}
