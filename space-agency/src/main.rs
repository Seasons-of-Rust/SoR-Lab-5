use personnel::{AstronautJob, Candidate};

fn main() {
    let mut candidates = Candidate::load_candidate_file();

    // Sort the list
    candidates.sort_by(|a, b| {
        let a_score = get_astronaut_score(a);
        let b_score = get_astronaut_score(b);

        b_score.cmp(&a_score)
    });
}

fn get_job_score(job: &AstronautJob) -> u32 {
    match job {
        AstronautJob::Biogeochemist => 251,
        AstronautJob::Biologist => 257,
        AstronautJob::Engineer => 263,
        AstronautJob::Geologist => 269,
        AstronautJob::Mechanic => 271,
        AstronautJob::Medic => 277,
        AstronautJob::RoverOp => 281,
        AstronautJob::Scientist => 283,
    }
}

fn get_astronaut_score(astronaut: &Candidate) -> u32 {
    let mut job_score = get_job_score(&astronaut.primary_job);

    if let Some(secondary_job) = &astronaut.secondary_job {
        job_score *= get_job_score(secondary_job);
    } else {
        job_score *= job_score;
    }

    job_score %= 577;

    ((job_score + astronaut.health as u32) * astronaut.age as u32) % 3929
}
