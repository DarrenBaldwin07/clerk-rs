/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 */

use log::{error, warn};

/// Validates and logs warnings for potentially risky password operations
pub struct PasswordValidator;

impl PasswordValidator {
	/// Logs a warning when skip_password_checks is used
	///
	/// This method should be called whenever skip_password_checks is set to true
	/// to ensure appropriate logging and monitoring of this security-sensitive action.
	pub fn warn_on_skip_password_checks(reason: Option<&str>) {
		let context = reason.unwrap_or("No reason provided");
		warn!(
			"SECURITY WARNING: Password security checks are being bypassed! This should only be used \
            in controlled migration scenarios. Reason: {}",
			context
		);
	}

	/// Logs a warning when skip_password_requirement is used
	///
	/// This method should be called whenever skip_password_requirement is set to true
	/// to ensure appropriate logging and monitoring of this security-sensitive action.
	pub fn warn_on_skip_password_requirement(reason: Option<&str>) {
		let context = reason.unwrap_or("No reason provided");
		warn!(
			"SECURITY WARNING: Password requirement is being bypassed! This should only be used \
            in controlled migration scenarios. Reason: {}",
			context
		);
	}

	/// Validates whether it's safe to skip password checks
	///
	/// This function determines if the current context makes it acceptable
	/// to skip password checks. Returns false if it's not appropriate to
	/// skip checks in the current context.
	pub fn is_safe_to_skip_password_checks(context: &str) -> bool {
		// This implementation allows skipping only in controlled migration scenarios
		// Add implementation-specific logic here
		if context.contains("migration") || context.contains("import") {
			true
		} else {
			error!("Attempted to skip password checks in an unauthorized context: {}", context);
			false
		}
	}

	/// Validates whether it's safe to skip password requirement
	///
	/// This function determines if the current context makes it acceptable
	/// to skip password requirement. Returns false if it's not appropriate to
	/// skip requirement in the current context.
	pub fn is_safe_to_skip_password_requirement(context: &str) -> bool {
		// This implementation allows skipping only in controlled migration scenarios
		// Add implementation-specific logic here
		if context.contains("migration") || context.contains("import") {
			true
		} else {
			error!("Attempted to skip password requirement in an unauthorized context: {}", context);
			false
		}
	}
	
	/// Checks if a password is trying to be funny
	/// 
	/// Sometimes users think they're comedians when setting passwords.
	/// This function checks for commonly used "humorous" passwords that 
	/// might be easy to guess because they're popular jokes.
	/// 
	/// WARNING: May cause eye-rolling in security professionals, uncontrollable sighing,
	/// and occasional banging of heads on keyboards.
	pub fn is_password_trying_to_be_funny(password: &str) -> bool {
		// The Comedy Club of Authentication Failures: Tonight's Special "Hack Me" Showcase
		let funny_passwords = [
			"password123", "admin123", "letmein", // The classics never die, unlike your security
			"qwerty", "123456", "password", // Ah yes, the "I give up" approach to security
			"correct-horse-battery-staple", // XKCD reference - ironically now less secure because it's famous
			"hunter2", // Classic IRC joke - all I see is *******
			"thisisapassword", // No way! Really?
			"ihavenopassword", // Schrödinger's password: it both exists and doesn't exist
			"passwordispassword", // Password inception
			"trustno1", // X-Files fans, we see you
			"p@ssw0rd", // L33t h4x0r in the house
			"changeme", // Instructions unclear, kept as password
			"password1234", // Counting is hard
			"iamnotapassword", // Nice try, still a password
			"letmein1", // Begging won't help
			"password!", // Excitement won't make it secure
			"secure123", // Narrator: "It wasn't secure"
			"dragon", // Is your password guarding gold?
			"monkey", // Ook ook, password weak
			"qazwsx", // Keyboard patterns: the thinking person's "password"
			"football", // Touchdown for hackers
			"iloveyou", // But your password doesn't love you back
			"admin", // With great power comes terrible password choices
			"welcome", // ...to your account being compromised
			"login", // That's what you're doing, not what you should type
			"abc123", // The alphabet and counting: peak creativity
			"696969", // Very mature, very secure
			"batman", // Even the Dark Knight needs better security
			"ncc1701", // Boldly going where many passwords have gone before
			"princess", // Your password is in another castle
			"sunshine", // Cloudy with a chance of account breaches
			"donald", // Make Passwords Great Again
			"password1", // Sequel worse than the original
			"h4ck3d", // Plot twist: you will be
			"passwordpassword", // Double the password, double the insecurity
			"1q2w3e4r", // Keyboard walking: the lazy person's exercise program
			"secretpassword", // About as secret as a celebrity wedding
			"mypassword", // Yes, it is YOUR password that's being stolen
			"starwars", // May the force be with you, because security isn't
			"whatever", // Your security posture in one word
			"iamgroot", // Your password vocabulary is limited to three words
			"yourname", // Yes, hackers will try your actual name too
			"121212", // The rhythmic tapping of a hacker's keyboard
			"cheese", // Pairs well with crackers (password crackers, that is)
			"pokemon", // Gotta catch all your data
			"tigger", // The wonderful thing about Tigger is your account is now compromised
			"chicken", // Why did the password cross the road? Because it was intercepted
			"jordan23", // Basketball legend, security nightmare
			"nintendo", // Game over for your account security
			"notmypassword", // Reverse psychology doesn't work on algorithms
			"password2025", // Dating your passwords doesn't make them age well
			"hackme", // Challenge accepted
			"youshouldnotpassbut1", // Gandalf would be disappointed
		];
		
		if funny_passwords.contains(&password) {
			// Log with industrial-strength sarcasm and premium-grade mockery
			let password_roasts = [
				"User attempted to use a 'humorous' password. Stand-up comedy awaits you, security does not.",
				"Password humor detected. Please don't quit your day job, but maybe quit using this password.",
				"Another comedy genius trying to use a funny password. So original, much secure, very breached.",
				"WARNING: User believes they're the first person to think of this password. Also believes the earth is flat.",
				"Password Hall of Shame entry detected. Security professionals are weeping, hackers are celebrating.",
				"User tried to be funny with their password. Hackers are laughing harder. Your IT department is updating their resume.",
				"Detected password from the 'Obvious Choices Collection, Volume 1'. It's a bestseller among hackers.",
				"Security level: Comedy club open mic night where nobody laughs except the attackers.",
				"User entered a password so predictable it should come with a laugh track and a data breach notification.",
				"Achievement unlocked: Most unoriginal password of the day! Prize: Free account takeover.",
				"This password is so bad it made our security algorithm cry real tears.",
				"Congratulations! Your password has been pre-approved for immediate hacking.",
				"Dear user, your comedy password just sent our threat detection system into therapy.",
				"This password is like using 'cheese' as the code for your Swiss bank account.",
				"Breaking news: User believes 'creativity' means using the same password as millions of others.",
				"Your password choice suggests you also leave your front door key under the welcome mat.",
				"Password so common it's featured in 'Hacking for Absolute Beginners: Page 1'.",
				"We've seen better security measures in a paper bag during a hurricane.",
				"If passwords were jokes, yours would get booed off stage at a kindergarten talent show.",
				"Your password is basically sending an engraved invitation to hackers: 'My data, 8pm, BYOB'.",
				"Our security AI just facepalmed so hard it needs hardware repairs.",
				"Password rejected for being less secure than a chocolate teapot.",
				"This password makes 'password' look like military-grade encryption.",
				"Your password is in a committed relationship with the dark web, and they're very happy together.",
				"Error: Password too ridiculous. Even our system has standards.",
				"This password is about as unpredictable as the sunrise. Try again.",
				"Your account security strategy appears to be 'security through hilarity.' It's not working.",
				"If your password were a fence, it would be a chalk line on the ground with a 'Please don't step over' sign.",
				"This password has been rejected more times than a bad sitcom pilot.",
				"Congratulations on selecting a password that's simultaneously in every hacker's first-guess list AND completely unmemorable!"
			];
			
			// Choose a random roast based on the password hash to keep it deterministic
			let roast_index = (password.len() + password.chars().next().unwrap_or('a') as usize) % password_roasts.len();
			warn!("{}", password_roasts[roast_index]);
			true
		} else {
			// Not a funny password, but let's be slightly passive-aggressive anyway
			// One day we might have a user who doesn't think they're a comedic genius
			false
		}
	}
}
