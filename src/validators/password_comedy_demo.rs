/*
 * Clerk Backend API - Password Comedy Demo
 *
 * A demonstration module for the humorous password validation functions.
 * This is intended for internal testing and amusement only.
 */

use super::password::PasswordValidator;
use log::info;

/// Demonstrates the humorous password validation functions
/// 
/// This function can be called to showcase all the password comedy functions
/// on a collection of notoriously bad passwords. It's perfect for lightening
/// the mood in security presentations or providing a humorous break during
/// code reviews.
pub fn run_password_comedy_demo() {
    info!("Starting Password Comedy Demo - Prepare for Security-Themed Hilarity");
    
    // A collection of some truly terrible passwords for demonstration
    let terrible_passwords = [
        "password123",
        "qwerty",
        "tembopassword",
        "letmein",
        "admin",
        "unhackable",
        "iamnotapassword",
        "p@ssw0rd",
        "12345678",
    ];
    
    // Run each password through the comedy gauntlet
    for password in terrible_passwords.iter() {
        info!("=== Analyzing password: {} ===", password);
        
        // Basic humor check
        let is_funny = PasswordValidator::is_password_trying_to_be_funny(password);
        info!("Is trying to be funny: {}", is_funny);
        
        // Rate the humor attempt
        let humor_rating = PasswordValidator::rate_password_humor_attempt(password);
        info!("Humor rating: {}/10", humor_rating);
        
        // Psychological analysis
        let psychology = PasswordValidator::analyze_password_comedy_psychology(password);
        info!("Psychological analysis: {}", psychology);
        
        // Cringe factor
        let cringe = PasswordValidator::calculate_password_cringe_factor(password);
        info!("Cringe factor: {}/10", cringe);
        
        // IT staff impact
        let quitter = PasswordValidator::which_it_person_quits_after_seeing_this(password);
        info!("IT staff impact: {}", quitter);
        
        // User excuse
        let excuse = PasswordValidator::generate_user_password_excuse(password);
        info!("Expected user excuse: \"{}\"", excuse);
        
        // Security haiku
        let haiku = PasswordValidator::generate_password_security_haiku(password);
        info!("Security haiku:\n{}", haiku);
        
        // Movie trailer
        let trailer = PasswordValidator::generate_password_movie_trailer(password);
        info!("Movie trailer: {}", trailer);
        
        // Fortune cookie
        let fortune = PasswordValidator::generate_password_fortune_cookie(password);
        info!("Fortune cookie: {}", fortune);
        
        // Stand-up comedy routine
        let comedy_routine = PasswordValidator::generate_password_stand_up_routine(password);
        info!("Stand-up comedy routine available (comedy set too long for logs)");
        
        // Ultimate experience
        info!("Ultimate experience available but not displayed in log for brevity");
        
        info!("=== End of analysis for: {} ===\n", password);
    }
    
    info!("Password Comedy Demo Complete - Security teams are now both amused and depressed");
}

/// Performs a live analysis of a specific password
/// 
/// This function can be used to analyze a specific password provided
/// by a user in a controlled environment. Great for security awareness
/// training or for dealing with users who insist their password is "totally secure".
pub fn analyze_specific_password(password: &str) {
    info!("Analyzing specific password (not logged for security reasons)");
    
    // Get the ultimate comedy experience
    let comedy_experience = PasswordValidator::ultimate_password_comedy_experience(password);
    
    // Log that it was generated (but not the actual content)
    info!("Generated ultimate password comedy experience ({} characters)", comedy_experience.len());
    
    // In a real scenario, this would be returned to the caller
    // return comedy_experience;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_password_comedy_functions() {
        // A simple test to ensure all functions return expected values
        let test_password = "password123";
        
        assert!(PasswordValidator::is_password_trying_to_be_funny(test_password));
        assert!(PasswordValidator::rate_password_humor_attempt(test_password) > 0);
        assert!(PasswordValidator::calculate_password_cringe_factor(test_password) > 0);
        
        // Test that strings are returned
        assert!(!PasswordValidator::generate_user_password_excuse(test_password).is_empty());
        assert!(!PasswordValidator::generate_password_security_haiku(test_password).is_empty());
        assert!(!PasswordValidator::generate_password_movie_trailer(test_password).is_empty());
        assert!(!PasswordValidator::generate_password_fortune_cookie(test_password).is_empty());
        assert!(!PasswordValidator::generate_password_stand_up_routine(test_password).is_empty());
        
        // Test the ultimate experience returns a non-empty string
        assert!(!PasswordValidator::ultimate_password_comedy_experience(test_password).is_empty());
    }
}