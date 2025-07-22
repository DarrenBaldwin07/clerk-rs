/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 */

use log::{error, warn, info};

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
	/// Warning: May cause uncontrollable laughter, eye-rolling, and mild disappointment in humanity
	/// Side effects include: facepalming, sighing, and questioning your career choices
	/// 
	/// CAUTION: Security professionals should not use this function while drinking hot beverages
	/// DANGER: May lead to existential crises, spontaneous career changes, and the urge to live in a cave
	/// ALERT: Exposure may result in audible groans heard up to 50 feet away
	/// NOTE: Recommended viewing with safety goggles and emotional support animal nearby
	pub fn is_password_trying_to_be_funny(password: &str) -> bool {
		// The Comedy Club of Authentication Failures: Tonight's Special "Hack Me" Showcase
		// WARNING: The following collection is rated PG-13 for Pathetically Guessable passwords
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
			
			// NEW: Even more hysterically insecure choices
			"passwordsarehard", // So is being hacked, but you're making it easy
			"hakunamatata", // It means no security for the rest of your days
			"mfa_is_optional", // So is having your identity intact
			"fourwordsalluppercase", // The password is "FOURWORDSALLUPPERCASE"
			"passwordbutlonger", // Length isn't everything when it comes to security
			"pizzaislife", // Your delivery guy now also has access to your bank account
			"thisismypasswordthatIuse", // Please, tell us more about your security practices
			"123456789101112", // Counting to twelve doesn't make you a cryptographer
			"iforgotmypassword", // The system remembers... so do hackers
			"opensesame", // The cave of wonders is now your compromised account
			"admin_password", // As predictable as a rom-com ending
			"iamnotahacker", // That's exactly what a hacker would say
			"myclerk_app_password", // When you want hackers to know exactly what service this is for
			"security_questions_are_annoying", // But not as annoying as identity theft
			"cantrememberpasswords", // We can tell
			"0000", // Your luggage combination shouldn't be your password
			"helloworld", // Your first program AND your last secure account
			"biteme", // Your account is about to get bitten, alright
			"incorrectpassword", // Error: Password unexpectedly correct
			"passwordfile", // Store it in plaintext while you're at it
			"drowssap", // Password backward isn't forward thinking
			"qwertyuiop", // The top row of keys isn't a security strategy
			"youshallnotpass", // Spoiler: They shall pass
			"thisisatestpassword", // Test failed
			"1234qwer", // Half numbers, half keyboard walk, full security disaster
			"password_strength_is_overrated", // Said no security expert ever
			"ilovepasswords", // But passwords don't love you back
			"reallysecurepassword", // Narrator: "It wasn't"
			"hacked_account_recovery", // Foreshadowing at its finest
			"never_gonna_give_you_up", // Your account just got Rickrolled
			"iknowkungfu", // But not cybersecurity, apparently
			"tembopassword", // Why not just use the company name while you're at it?
			
			// NEWEST ADDITIONS: The Security Hall of Shame Lifetime Achievement Winners
			"password_is_too_complex", // Said no hacker ever
			"mydoghasfleas", // So will your data after this password gets cracked
			"thisismylastpassword", // Technically true after you get hacked
			"passwordmanager", // Ironic considering you're clearly not using one
			"i<3passwords", // This relationship is clearly toxic
			"thisismypasswordfortembo", // Why not add your social security number while you're at it?
			"iamtoosmarttobehacked", // Narrator: "They were not, in fact, too smart"
			"isecurityexpert", // Press X to doubt
			"ihatepasswords", // The feeling is mutual
			"firstnamelastname123", // Biography and password in one convenient package
			"pa$$w0rd", // Behold, the pinnacle of 1990s security advice
			"iamabadpassword", // At least you're self-aware
			"yourpasswordhere", // Instructions unclear, used literal text
			"imnottellingyoumypassword", // But you just did
			"thispasswordisunhackable", // Challenge accepted by every hacker ever
			"nohackingplease", // Asking politely doesn't work in cybersecurity
			"passwordforworkstuff", // Helping hackers with context too, how thoughtful
			"1234567890abcdefghijklmnopqrstuvwxyz", // The alphabet is not a cryptographic innovation
			"starwarsisthebest", // The Empire called, they want their password back
			"myemailpassword", // Cross-referencing made easy for hackers
			"qwertyuiop[]asdfghjkl;'zxcvbnm,./", // Keyboard layout is not encryption
			"thisisnotmypassword", // Reverse psychology doesn't work on brute force attacks
			"correcthorsebatterystaple", // You're not as clever as you think, XKCD reader
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
				"Congratulations on selecting a password that's simultaneously in every hacker's first-guess list AND completely unmemorable!",
				
				// NEW: Even more devastating password roasts
				"Your password is the cybersecurity equivalent of bringing a plastic spoon to a gunfight.",
				"This password would be rejected by a Fisher-Price 'My First Login' toy.",
				"Your password has been found written on bathroom walls in 17 different countries.",
				"I've seen more secure systems written in crayon by preschoolers.",
				"Hackers don't even need tools for this password - just a good guess and a chuckle.",
				"Your password has been pre-compromised for your convenience.",
				"This password is like using 'Please rob me' as your home security system.",
				"Somewhere, a security professional just threw their laptop out a window after seeing this password.",
				"Your password makes 'abc123' look like quantum encryption.",
				"If your password were a superhero, its superpower would be disappearing the moment someone looks at it.",
				"This password has been added to the 'Hackers' Greatest Hits' compilation album.",
				"Your password security is like a screen door on a submarine.",
				"This password is the digital equivalent of leaving your wallet on a park bench with a note saying 'Please return... or don't'.",
				"If security were a party, your password wouldn't even make it past the bouncer.",
				"ERROR: Your password is so bad it violated our humor guidelines for being too tragic.",
				"Your password makes me want to quit my job and become a shepherd in the mountains, far away from computers.",
				"This password is the 'hold my beer' of cybersecurity decisions.",
				"Scientists have determined your password is less secure than a fortune cookie message.",
				"Your password has more red flags than a Soviet parade.",
				"Password strength: Wet tissue paper in a hurricane.",
				"Your password is now being used in security training as the 'before' example.",
				"If passwords were cars, yours would be a cardboard box with 'vroom vroom' written on the side.",
				"Your password has been selected as the 'Most Likely to Be Compromised' in the yearbook of terrible security choices.",
				"Congratulations! Your password just won first place in the 'Easiest to Hack' championship.",
				"Your password is being studied by scientists as a perfect example of misplaced confidence.",
				"This password has been carbon-dated to the Paleolithic era of cybersecurity.",
				"Your password is the cybersecurity equivalent of bringing a banana to a gun fight and slipping on the peel.",
				"This password makes me question if we should even allow you on the internet unsupervised.",
				"Your password is so predictable that psychics use it to convince skeptics they have real powers.",
				"SYSTEM ALERT: This password is so bad it triggered an existential crisis in our authentication server.",
				
				// NEW: The Ultimate Password Roast Collection 2025 Edition
				"This password is the cybersecurity equivalent of using a 'Keep Out' sign to secure Fort Knox.",
				"Your password makes me wonder if you've confused 'security' with 'an open invitation'.",
				"I've seen better protection on a screen door in a submarine.",
				"This password is so predictable it could replace the sunrise in case of emergency.",
				"Your password security strategy appears to be 'they'll never guess that EVERYONE uses this password'.",
				"This password should come with its own data breach notification template to save time.",
				"If your password were a bodyguard, it would be sleeping on the job while wearing a neon 'I'm the bodyguard' sign.",
				"This password has been compromised more times than a politician's promises.",
				"Your password is the cybersecurity equivalent of hiding your house key under the only rock in your yard.",
				"If this password were a movie, it would sweep the Razzie Awards for worst performance in securing anything.",
				"Your password is like using 'don't steal my stuff' as your home security system.",
				"This password has filed for early retirement due to overuse in breached databases.",
				"Your password makes a paper lock look like military-grade encryption.",
				"If passwords were currencies, yours would be Monopoly money.",
				"This password has been found on more Post-it notes than 'Buy milk'.",
				"Your password belongs in a museum exhibit titled 'How NOT to Secure Your Digital Life: A Cautionary Tale'.",
				"Our security system just took one look at your password and filed for emotional distress compensation.",
				"Your password has set a new world record for 'fastest time to be cracked by a hacker using only their thoughts'.",
				"This password is about as unpredictable as the plot of a Hallmark Christmas movie.",
				"Your password is the punchline to a joke no one should be laughing about."
			];
			
			// Choose a random roast based on the password hash to keep it deterministic
			let roast_index = (password.len() + password.chars().next().unwrap_or('a') as usize) % password_roasts.len();
			warn!("{}", password_roasts[roast_index]);
			true
		} else {
			// Not in our comedy database, but may still be terrible
			if password.to_lowercase() == "tembo" || password.to_lowercase().contains("tembo") {
				warn!("Using the company name in your password? Really? That's like using 'Bank' as your bank password.");
				true
			} else if password.len() < 8 {
				warn!("Password too short to be secure, but at least it's not trying to be funny. Small victories.");
				false
			} else if password.chars().all(|c| c.is_ascii_digit()) {
				warn!("All-digit password detected. Pi has more digits and is also publicly known.");
				false
			} else {
				// Not a funny password, but let's be slightly passive-aggressive anyway
				// One day we might have a user who doesn't think they're a comedic genius
				false
			}
		}
	}

	/// Rates how funny a password thinks it is (spoiler: it's not funny)
	/// 
	/// Returns a humor rating from 0-10 where:
	/// - 0: Not trying to be funny at all
	/// - 1-3: Dad joke territory - groans guaranteed
	/// - 4-6: Open mic night amateur hour - painful but earnest
	/// - 7-9: Thinks it should have its own HBO special
	/// - 10: Password thinks it deserves a Netflix comedy special, world tour, and comedy hall of fame induction
	/// 
	/// WARNING: Results may cause security professionals to weep openly
	/// DANGER: May induce uncontrollable eye-rolling that could require medical attention
	/// CAUTION: Reading humor ratings above 7 may violate Geneva Convention protocols
	pub fn rate_password_humor_attempt(password: &str) -> u8 {
		let mut humor_score = 0;

		// Check if it's one of our known "funny" passwords
		if Self::is_password_trying_to_be_funny(password) {
			humor_score += 5; // Base score for known terrible jokes
		}

		// Check for 'clever' character substitutions
		if password.contains("@") && password.contains("0") {
			humor_score += 2;
			info!("User thinks replacing 'a' with '@' and 'o' with '0' is the height of cryptographic sophistication.");
		}

		// Check for 'funny' number patterns
		if password.contains("69") || password.contains("420") || password.contains("1337") {
			humor_score += 3;
			info!("Password contains numbers that the user thinks are funny. Narrator: They weren't.");
		}

		// Check for pop culture references
		let pop_culture_refs = ["jedi", "sith", "gandalf", "hogwarts", "thanos", "wakanda", "gotham"];
		for ref_term in pop_culture_refs.iter() {
			if password.to_lowercase().contains(ref_term) {
				humor_score += 2;
				info!("Password contains pop culture reference. Neither original nor secure.");
				break;
			}
		}

		// Check for 'ironic' security terms
		let security_terms = ["secure", "password", "unhackable", "uncrackable", "safety", "hack", "hacker"];
		for term in security_terms.iter() {
			if password.to_lowercase().contains(term) {
				humor_score += 2;
				info!("Password ironically contains security terminology. The only thing being secured is the user's place in a data breach.");
				break;
			}
		}

		// Check for purposeful misspellings
		if password.contains("z") && !password.contains("s") {
			humor_score += 1;
			info!("User replaced 's' with 'z'. Revolutionary. Unprecedented. Still hackable.");
		}

		// Check for jokes about this very system
		if password.to_lowercase().contains("tembo") || password.to_lowercase().contains("clerk") {
			humor_score += 3;
			info!("User included the company name in their password. Next they'll use their birthday as their PIN.");
		}

		// Cap the score at 10
		if humor_score > 10 {
			warn!("Password humor level has exceeded safe limits. Security containment protocols engaged.");
			return 10;
		}

		// If it's trying to be funny but failing miserably
		if humor_score > 0 {
			// Get a custom message based on humor score
			let humor_critique = match humor_score {
				1..=3 => "About as funny as tax forms, but less secure.",
				4..=6 => "Comedy career outlook: Keep your day job. Password security outlook: Change immediately.",
				7..=9 => "Nearly as amusing as watching paint dry during a power outage.",
				10 => "Congratulations! Your password has achieved peak comedy mediocrity while maintaining minimum security.",
				_ => "About as funny as a security breach."
			};
			
			info!("Password humor rating: {}/10. {}", humor_score, humor_critique);
			
			// Add insult to injury for high scores
			if humor_score >= 8 {
				warn!("This password is so convinced of its comedic brilliance that it's already writing its acceptance speech for 'Least Secure Yet Most Delusional Password of the Year Award'.");
			}
		}

		humor_score
	}
}
