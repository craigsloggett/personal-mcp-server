struct Args {
    program_name: String,
}

struct Env {
    config_home: String,
}

pub struct ServerConfig {
    args: Args,
    env: Env,
}

impl ServerConfig {
    pub fn build() -> Result<ServerConfig, Box<dyn std::error::Error>> {
        Ok(Self { args: Self::from_args()?, env: Self::from_env()? })
    }

    pub fn program_name(&self) -> &str {
        &self.args.program_name
    }

    pub fn config_home(&self) -> &str {
        &self.env.config_home
    }

    fn from_args() -> Result<Args, &'static str> {
        let argv: Vec<String> = std::env::args().collect();

        Ok(Args { program_name: argv[0].clone() })
    }

    fn from_env() -> Result<Env, std::env::VarError> {
        let config_home = std::env::var("XDG_CONFIG_HOME")?;

        Ok(Env { config_home })
    }
}
