use std::{
    env,
    io::Write,
    process::{Command, Stdio},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <status_code>", args[0]);
        return;
    }

    let url = format!("https://http.cat/{}", &args[1]);

    let mut easy = curl::easy::Easy::new();
    easy.url(&url).unwrap();

    let mut data = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    if easy.response_code().unwrap() != 200 {
        eprintln!(
            "HTTP request failed with status code: {}",
            easy.response_code().unwrap()
        );
        return;
    }

    let imgcat_command = Command::new("/Users/arvidsson/.iterm2/imgcat")
        .stdin(Stdio::piped())
        .spawn();

    match imgcat_command {
        Ok(mut child) => {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(&data).unwrap();
            }
            let status = child.wait().unwrap();
            if !status.success() {
                eprintln!("Failed to display image with imgcat: {:?}", status.code());
            }
        }
        Err(err) => {
            eprintln!("Failed to run imgcat: {:?}", err);
        }
    }
}
