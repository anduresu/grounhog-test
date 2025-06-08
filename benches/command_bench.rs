use criterion::{black_box, criterion_group, criterion_main, Criterion};
use groundhog::cli::commands::explain;

fn bench_explain_command(c: &mut Criterion) {
    c.bench_function("explain_command_no_topic", |b| {
        b.iter(|| {
            let result = explain::execute(black_box(None));
            black_box(result)
        })
    });

    c.bench_function("explain_command_with_topic", |b| {
        b.iter(|| {
            let result = explain::execute(black_box(Some("rust".to_string())));
            black_box(result)
        })
    });
}

fn bench_config_loading(c: &mut Criterion) {
    use groundhog::infrastructure::Config;
    
    c.bench_function("config_default", |b| {
        b.iter(|| {
            let config = Config::default();
            black_box(config)
        })
    });
}

fn bench_error_handling(c: &mut Criterion) {
    use groundhog::infrastructure::error::{CommandError, GroundhogError};
    
    c.bench_function("error_creation", |b| {
        b.iter(|| {
            let error = GroundhogError::Command(CommandError::NotFound {
                command: black_box("invalid".to_string()),
            });
            black_box(error)
        })
    });

    c.bench_function("error_user_message", |b| {
        let error = GroundhogError::Command(CommandError::NotFound {
            command: "invalid".to_string(),
        });
        b.iter(|| {
            let message = error.user_message();
            black_box(message)
        })
    });
}

criterion_group!(benches, bench_explain_command, bench_config_loading, bench_error_handling);
criterion_main!(benches); 