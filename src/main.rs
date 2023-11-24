use color_eyre::Report;
use html_node::{
    typed::{elements::*, html},
    Node,
};

fn template(inner: Node) -> Node {
    let page_style_tag = html! {
        <style>
          body {
            text-align: center;
            font-family: courier, monospace;
            font-size: 2rem;
            margin: 20px;
            background-color: #000000;
            padding: 10px;
            color: #fff;
          }
          .justify {
            text-align: justify;
            text-justify: inter-word;
          }
          a, .white, h1, h2 {
            color: white;
          }
          .slogan {
            font-family: serif;
            font-size: x-large;
            font-weight: bold;
          }
          .narrow {
             max-width: 50%;
          }
          h1 {
            max-width: 40rem;
            line-height: 3rem;
          }
          .titles {
            font-size: 3rem;
            font-weight: bold;
          }
        img{
            width:100%;
            max-width:1000px;
        }
        </style>
    };

    html! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
               <meta http-equiv="x-clacks-overhead" content="GNU Terry Pratchett" />
               <link rel="icon" href="favicon.png"/>

               <meta charset="utf-8"/>
                <meta name="description" content="An urban fantasy podcast of tape recordings by the curator of a secretive london-based art auction house."/>

               <meta content="width=device-width, initial-scale=1" name="viewport"/>
               <title>"The Phosphene Catalogue Podcast"</title>

               { page_style_tag }

            </head>

            <body>

                <div class="titles">
                    <a href="">Listen</a>
                    <div></div>
                    <a href="">Mastodon</a>
                    <div></div>
                    <a href="https://discord.gg/mCY2bBmDKZ">Discord</a>
                    <div></div>
                    <a href="">Patreon</a>
                </div>

                <br/>
                <br/>

                <a href="index.html">
                    <img alt="A photo of an art catelogue cover" src="logo.png" width="50%"/>
                </a>

               {inner}

            </body>
        </html>
    }
}

/// NOTE: the widget requires https to load
fn soundcloud_widget() -> Node {
    html_node::html! {
        <iframe width="100%" height="166" scrolling="no" frameborder="no" allow="autoplay" src="https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/1673705718&color=%23ff5500&auto_play=false&hide_related=true&show_comments=false&show_user=true&show_reposts=false&show_teaser=false">
            </iframe>
            <div style="font-size: 10px; color: #cccccc;line-break: anywhere;word-break: normal;overflow: hidden;white-space: nowrap;text-overflow: ellipsis; font-family: Interstate,Lucida Grande,Lucida Sans Unicode,Lucida Sans,Garuda,Verdana,Tahoma,sans-serif;font-weight: 100;">
            <a href="https://soundcloud.com/namtao" title="namtao" target="_blank" style="color: #cccccc; text-decoration: none;">namtao</a>
            <a href="https://soundcloud.com/namtao/the-phosphene-catalogue-pilot" title="The Phosphene Catalogue [PILOT]" target="_blank" style="color: #cccccc; text-decoration: none;">The Phosphene Catalogue [PILOT]</a>
            </div>
    }
}

fn index() -> Node {
    template(html! {
          <div class="slogan"> "We see light where others see only darkness." </div>
          <br/>
          <br/>
        <div class="justify">
          "An urban fantasy podcast of tape recordings by the curator of a secrative London-based art auction house."
          <br/>
          <br/>
          <b>The Phosphene Catalogue</b> is a 1970s mail-order catalogue, specialising in those items that cannot be sold at other auction houses: Paintings of lost origin, statues that are "too grotesque" for public display, and books better left unread...
          </div>
      <div class="narrow">
      </div>
    <br/>
    <br/>

        <b><a href="">"Listen to the pilot here"</a></b>
        { soundcloud_widget() }
        <br/>
        <br/>
    })
}

type Router<'a> = Vec<(&'a str, fn() -> Node)>;

fn build(pages: Router) -> Result<(), Report> {
    for (page, fun) in pages {
        std::fs::write(page, fun().to_string())?;
    }
    Ok(())
}

fn main() -> Result<(), Report> {
    std::fs::create_dir_all("docs")?;
    let _ = build(vec![("docs/index.html", index)]);
    println!("Built site OK!");
    Ok(())
}
