use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, Write};

struct Post {
    title: String,
    intro: String,
    bullet_points: Vec<String>,
    link: String,
    file_name: String,
}

impl Post {
    fn new() -> Self {
        Self {
            title: String::new(),
            intro: String::new(),
            bullet_points: Vec::new(),
            link: String::new(),
            file_name: String::new(),
        }
    }

    fn collect_input(&mut self) {
        let mut input = String::new();

        println!("Digite o título:");
        io::stdin()
            .read_line(&mut input)
            .expect("Erro ao ler o título");
        self.title = input.trim().to_string();
        input.clear();

        println!("Digite a introdução:");
        io::stdin()
            .read_line(&mut input)
            .expect("Erro ao ler a introdução");
        self.intro = input.trim().to_string();
        input.clear();

        println!("Digite os pontos (digite 'fim' para terminar):");
        loop {
            io::stdin()
                .read_line(&mut input)
                .expect("Erro ao ler os pontos");
            let point = input.trim().to_string();
            if point.to_lowercase() == "fim" {
                break;
            }
            self.bullet_points.push(point);
            input.clear();
        }

        println!("Digite o link:");
        io::stdin()
            .read_line(&mut input)
            .expect("Erro ao ler o link");
        self.link = input.trim().to_string();
        input.clear();

        println!("Digite o nome do arquivo para salvar o post (com extensão .txt):");
        io::stdin()
            .read_line(&mut input)
            .expect("Erro ao ler o nome do arquivo");
        self.file_name = input.trim().to_string();
    }

    fn random_emoji<'a>(&'a self, emojis: &[&'a str]) -> &str {
        let mut rng = rand::thread_rng();
        emojis.choose(&mut rng).unwrap()
    }

    fn generate_linkedin_post(&self) -> String {
        let rocket_emojis = ["🚀", "🛫", "🛰️"];
        let magnifying_glass_emojis = ["🔍", "🔎", "🧐"];
        let finger_pointing_emojis = ["👉", "➡️"];
        let bell_emojis = ["🔔", "📣"];
        let speech_balloon_emojis = ["💬", "🗨️", "🗯️"];

        format!(
            "{} **{}** {}\n\n{}\n\n{} **Neste post você vai encontrar:**\n{}\n\n{} Leia o post completo aqui: {}\n\n{} Não se esqueça de comentar e compartilhar suas experiências! Vamos aprender juntos! {}\n",
            self.random_emoji(&rocket_emojis),
            self.title,
            self.random_emoji(&rocket_emojis),
            self.intro,
            self.random_emoji(&magnifying_glass_emojis),
            self.bullet_points.iter().map(|p| format!("- {}", p)).collect::<Vec<_>>().join("\n"),
            self.random_emoji(&finger_pointing_emojis),
            self.link,
            self.random_emoji(&bell_emojis),
            self.random_emoji(&speech_balloon_emojis),
        )
    }

    fn generate_instagram_post(&self) -> String {
        let rocket_emojis = ["🚀", "🛫", "🛰️"];
        let magnifying_glass_emojis = ["🔍", "🔎", "🧐"];
        let finger_pointing_emojis = ["👉", "➡️"];
        let bell_emojis = ["🔔", "📣"];
        let speech_balloon_emojis = ["💬", "🗨️", "🗯️"];

        format!(
            "{} **{}** {}\n\n{}\n\n{} **Neste post você vai encontrar:**\n{}\n\n{} Leia o post completo aqui: {}\n\n{} Não se esqueça de comentar e compartilhar suas experiências! Vamos aprender juntos! {}\n#fullstack #development #programming #reactjs #nodejs",
            self.random_emoji(&rocket_emojis),
            self.title,
            self.random_emoji(&rocket_emojis),
            self.intro,
            self.random_emoji(&magnifying_glass_emojis),
            self.bullet_points.iter().map(|p| format!("- {}", p)).collect::<Vec<_>>().join("\n"),
            self.random_emoji(&finger_pointing_emojis),
            self.link,
            self.random_emoji(&bell_emojis),
            self.random_emoji(&speech_balloon_emojis),
        )
    }

    pub fn save_post(&self, content: &str) -> io::Result<()> {
        let mut file = File::create(&self.file_name)?;
        file.write_all(content.as_bytes())?;
        println!("Post salvo em {}", self.file_name);
        Ok(())
    }
}

pub fn write_linkedin_post() -> io::Result<()> {
    let mut post = Post::new();
    post.collect_input();

    // Gerar post para LinkedIn
    let linkedin_post = post.generate_linkedin_post();
    post.save_post(&linkedin_post)?;

    Ok(())
}
pub fn write_instagram_post() -> io::Result<()> {
    let mut post = Post::new();
    post.collect_input();

    // Gerar post para Instagram
    let instagram_post = post.generate_instagram_post();
    post.save_post(&instagram_post)?;

    Ok(())
}
