use color_eyre::Report;
use html_to_string_macro::html;

fn template(inner: String) -> String {
    let page_style = r#"
      body {
        font-family: monospace;
        font-size: large;
        margin: 20px;
        background-color: #000000;
        padding: 10px;
        color: #fff;
        //background-image: url('stars.jpeg');
        //background-repeat: no-repeat;
        //background-attachment: fixed;
        //background-size: cover;
      }
      a, .white, h1, h2 {
        color: white;
      }
      .slogan {
        font-family: cursive;
        font-size: x-large;
        font-weight: bold;
      }
      }
      .narrow {
         max-width: 50%;
      }
      h1 {
        max-width: 40rem;
        line-height: 3rem;
      }
"#;
    html! {
           <!DOCTYPE html>
           <html lang="en">

           <head>
               <meta http-equiv="x-clacks-overhead" content="GNU Terry Pratchett" />
               <link rel="icon" href="favicon.png"/>

               <meta charset="utf-8"/>
                <meta name="description" content="An urban fantasy podcast of tape recordings by the curator of a secrative london-based art auction house."/>

               <meta content="width=device-width, initial-scale=1" name="viewport"/>
               <title>"The Phosphene Catalogue Podcast"</title>

               <style>

               {page_style}

               </style>

           </head>

    <body class="glow">
      <center>
        <a href="">"Listen"</a>"&nbsp; "
        <a href="">"Mastodon"</a>"&nbsp; "
        <a href="https://discord.gg/mCY2bBmDKZ">"Discord"</a>"&nbsp; "
        <a href="">"Patreon"</a>"&nbsp; "
        <br/>
        <br/>


        <a href="index.html">
        <img alt="A photo of an art catelogue cover" src="logo.jpg" width="50%"/>
        </a>
    </center>
           {inner}

    </body>
    </html>
    }
}

fn index() -> String {
    template(html! {
    <center>
          <div class="slogan"> "We see light where others see only darkness." </div>
          <br/>
          <br/>
          "An urban fantasy podcast of tape recordings by the curator of a secrative london-based art auction house."
          <br/>
          <br/>
          "<i>The Phosphene Catalogue</i> is a mail-order catalogue from the 1970s, specialising in those items that cannot be sold at other auction houses: Paintings of lost origin, statues that are 'too grotesque' for public display, and books better left unread."
      <div class="narrow">
      </div>
    <br/>
    <br/>

        <a href="">"Listen to the pilot here"</a>
        <br/>
        <br/>
    </center>
    })
}

fn build(pages: Vec<(&str, fn() -> String)>) -> Result<(), Report> {
    for (page, fun) in pages {
        std::fs::write(page, fun())?;
    }
    Ok(())
}

fn main() -> Result<(), Report> {
    std::fs::create_dir_all("docs")?;
    let _ = build(vec![("docs/index.html", index)]);
    println!("Built site OK!");
    Ok(())
}
