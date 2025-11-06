
#[derive(Clone)]
struct Job {
    id: u32,
    arrive_time: u32,
    burst_time: u32,
}

fn order_by_arrive_time(jobs: &mut Vec<Job>) {
    jobs.sort_by(|a, b| a.arrive_time.cmp(&b.arrive_time));
}

fn sjf_algorithm(mut jobs: Vec<Job>) {
    let n = jobs.len();
    let mut completed = vec![false; n];
    let mut current_time = 0;
    let mut completed_jobs = 0;

    let mut waiting_times = vec![0; n];
    let mut turnaround_times = vec![0; n];

    order_by_arrive_time(&mut jobs);

    println!("Orden de ejecución (SJF):");

    while completed_jobs < n {
        let mut idx_min: Option<usize> = None;
        let mut min_burst = u32::MAX;

        for i in 0..n {
            if jobs[i].arrive_time <= current_time && !completed[i] {
                if jobs[i].burst_time < min_burst {
                    min_burst = jobs[i].burst_time;
                    idx_min = Some(i);
                }
            }
        }

        if idx_min.is_none() {
            current_time += 1;
            continue;
        }

        let idx = idx_min.unwrap();
        let job = &jobs[idx];

        println!("Proceso P{}", job.id);

        waiting_times[idx] = current_time.saturating_sub(job.arrive_time);
        current_time += job.burst_time;
        turnaround_times[idx] = current_time - job.arrive_time;

        completed[idx] = true;
        completed_jobs += 1;
    }

    let total_wait: u32 = waiting_times.iter().sum();
    let total_turnaround: u32 = turnaround_times.iter().sum();

    for i in 0..n {
        println!(
            "P{}       | {:>7} | {:>9} | {:>6} | {:>8}",
            jobs[i].id,
            jobs[i].arrive_time,
            jobs[i].burst_time,
            waiting_times[i],
            turnaround_times[i]
        );
    }

    println!(
        "\nTiempo promedio de espera: {:.2}",
        total_wait as f32 / n as f32
    );
    println!(
        "Tiempo promedio de retorno: {:.2}",
        total_turnaround as f32 / n as f32
    );
}

fn main() {
    let processes = vec![
        Job { id: 1, arrive_time: 0, burst_time: 8 },
        Job { id: 2, arrive_time: 1, burst_time: 4 },
        Job { id: 3, arrive_time: 2, burst_time: 9 },
        Job { id: 4, arrive_time: 3, burst_time: 5 },
    ];

    sjf_algorithm(processes);
}
