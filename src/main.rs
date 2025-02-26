use clap::Parser;
use ollama_rs::{
    Ollama,
    generation::{completion::request::GenerationRequest, options::GenerationOptions},
};
use tokio::io::{self, AsyncWriteExt};
use tokio_stream::StreamExt;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Opts {
    /// Define the model to run on Ollama.
    #[arg(short, long, default_value = "phi4")]
    model: String,
    /// Set the context length the model will digest.
    /// Higher values create better output, but require beefier computers.
    #[arg(short, long, default_value_t = 4096)]
    context: u64,
    #[arg(short, long, default_value_t = 0.6)]
    temperature: f32,
    #[arg(
        short,
        long,
        default_value = "You are a command-line program that takes an input and provides an output ONLY. Give me only the output, without any additional labels (e.g., 'Output' or 'Result'). The output should be usable as input in another program that is not an LLM. Avoid unnecessary chat. No preamble, get straight to the point. Generate a text response suitable for downstream processing by another program. Do not change the content of the input unless specifically asked to. Do not repeat back the input."
    )]
    system_prompt: String,
    /// The actual prompt for the LLM.
    prompt: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Opts::parse();

    let ollama = Ollama::default();

    let mut stream = ollama
        .generate_stream(
            GenerationRequest::new(args.model, args.prompt)
                .system(args.system_prompt)
                .options(
                    GenerationOptions::default()
                        .num_ctx(args.context)
                        .temperature(args.temperature),
                ),
        )
        .await?;

    let mut stdout = io::stdout();
    while let Some(res) = stream.next().await {
        let responses = res?;
        for resp in responses {
            stdout.write_all(resp.response.as_bytes()).await?;
            stdout.flush().await?;
        }
    }

    Ok(())
}
