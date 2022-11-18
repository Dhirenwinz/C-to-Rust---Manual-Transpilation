mod positions;
use std::ops::Indexmut;
use std::fs::OpenOptions;
use libc::{strpbrk, strchr, strcmp, getopt_long, atoi, strtod};
use std::ptr::{null, null_mut};
use std::char::from_u32;
/* Enumeration of the possible boolean options.  */

enum OptionType
{
  /* --- Input file interpretation --- */

  /* Handle user-defined type structured keyword input.  */
  TYPE = 1 << 0,

  /* Ignore case of ASCII characters.  */
  UPPERLOWER = 1 << 1,

  /* --- Language for the output code --- */

  /* Generate K&R C code: no prototypes, no const.  */
  KRC = 1 << 2,

  /* Generate C code: no prototypes, but const (user can #define it away).  */
  C  = 1 << 3,

  /* Generate ISO/ANSI C code: prototypes and const, but no class.  */
  ANSIC  = 1 << 4,

  /* Generate C++ code: prototypes, const, class, inline, enum.  */
  CPLUSPLUS = 1 << 5,

  /* --- Details in the output code --- */

  /* Assume 7-bit, not 8-bit, characters.  */
  SEVENBIT = 1 << 6,

  /* Generate a length table for string comparison.  */
  LENTABLE = 1 << 7,

  /* Generate strncmp rather than strcmp.  */
  COMP = 1 << 8,

  /* Make the generated tables readonly (const).  */
  CONST = 1 << 9,

  /* Use enum for constants.  */
  ENUM = 1 << 10,

  /* Generate #include statements.  */
  INCLUDE = 1 << 11,

  /* Make the keyword table a global variable.  */
  GLOBAL = 1 << 12,

  /* Use NULL strings instead of empty strings for empty table entries.  */
  NULLSTRINGS = 1 << 13,

  /* Optimize for position-independent code.  */
  SHAREDLIB = 1 << 14,

  /* Generate switch output to save space.  */
  SWITCH = 1 << 15,

  /* Don't include user-defined type definition in output -- it's already
     defined elsewhere.  */
  NOTYPE = 1 << 16,

  /* --- Algorithm employed by gperf --- */

  /* Use the given key positions.  */
  POSITIONS = 1 << 17,

  /* Handle duplicate hash values for keywords.  */
  DUP = 1 << 18,

  /* Don't include keyword length in hash computations.  */
  NOLENGTH = 1 << 19,

  /* Randomly initialize the associated values table.  */
  RANDOM = 1 << 20,

  /* --- Informative output --- */

  /* Enable debugging (prints diagnostics to stderr).  */
  DEBUG = 1 << 21
}




/* Class manager for gperf program Options.  */
struct Options {

    /* Records count of command-line arguments.  */
    _argument_count: i32,
    
    /* Stores a pointer to command-line argument vector.  */
    _argument_vector: *mut *mut char,
    
    /* Holds the boolean options.  */
    _option_word: i32,
    
    /* Name of input file.  */
    _input_file_name: *mut char,
    
    /* Name of output file.  */
    _output_file_name: *mut char,
    
    /* The output language.  */
    _language: *const char,
    
    /* Jump length when trying alternative values.  */
    _jump: i32,
    
    /* Initial value for asso_values table.  */
    _initial_asso_value: i32,
    
    /* Number of attempts at finding good asso_values.  */
    _asso_iterations: i32,
    
    /* Number of switch statements to generate.  */
    _total_switches: i32,
    
    /* Factor by which to multiply the generated table's size.  */
    _size_multiple: f32,
    
    /* Names used for generated lookup function.  */
    _function_name: *const char,
    
    /* Name used for keyword key.  */
    _slot_name: *const char,
    
    /* Suffix for empty struct initializers.  */
    _initializer_suffix: *const char,
    
    /* Name used for generated C++ class.  */
    _class_name: *const char,
    
    /* Name used for generated hash function.  */
    _hash_name: *const char,
    
    /* Name used for hash table array.  */
    _wordlist_name: *const char,
    
    /* Name used for length table array.  */
    _lengthable_name: *const char,
    
    /* Name used for the string pool.  */
    _stringpool_name: *const char,
    
    /* Separates keywords from other attributes.  */
    _delimiters: *const char,
    
    /* Contains user-specified key choices.  */
    _key_positions: Positions

}

/* Global option coordinator for the entire program.  */
static option: Options;

/* Records the program name.  */
static program_name: *const char;

/* Size to jump on a collision.  */
const DEFAULT_JUMP_VALUE: i32 = 5;



/* Default name for generated lookup function.  */
static temp: Vec<char> = (String::from("in_word_set")).chars().collect();
const DEFAULT_FUNCTION_NAME: *const char = &temp[0] as *const char; 

/* Default name for the key component.  */
static temp1: Vec<char> = (String::from("name")).chars().collect();
const DEFAULT_SLOT_NAME: *const char = &mut temp1[0] as *const char;


/* Default struct initializer suffix.  */
static temp2: Vec<char> = (String::from("")).chars().collect();
const DEFAULT_INITIALIZER_SUFFIX: *const char = &mut temp2[0] as *const char;


/* Default name for the generated class.  */
static temp3: Vec<char> = (String::from("Perfect_Hash")).chars().collect();
const DEFAULT_CLASS_NAME: *const char = &mut temp3[0] as *const char;

/* Default name for generated hash function. */
static temp4: Vec<char> = (String::from("hash")).chars().collect();
const DEFAULT_HASH_NAME: *const char = &mut temp4[0] as *const char;

/* Default name for generated hash table array.  */
static temp5: Vec<char> = (String::from("wordlist")).chars().collect();
const DEFAULT_WORDLIST_NAME: *const char = &mut temp5[0] as *const char;

/* Default name for generated length table array.  */
static temp6: Vec<char> = (String::from("lengthable")).chars().collect();
const DEFAULT_LENGTHTABLE_NAME: *const char = &mut temp6[0] as *const char;

/* Default name for string pool.  */
static temp7: Vec<char> =  (String::from("stringpool")).chars().collect();
const DEFAULT_STRINGPOOL_NAME: *const char = &mut temp7[0] as *const char;

/* Default delimiters that separate keywords from their attributes.  */
static temp8: Vec<char> =  (String::from(",")).chars().collect();
const DEFAULT_DELIMITERS: *const char = &mut temp8[0] as *const char;


const CHAR_MAX: u32 = 127;


/* Parses the command line Options and sets appropriate flags in option_word.  */

struct option {
    name: *const char,
    has_arg: i32,
    flag: *mut i32,
    val: i32
}

static long_options: [option;42] =
[
  option{ name: "output-file", has_arg: 1, flag: NULL, val: CHAR_MAX + 1 },
  option{ name: "ignore-case", has_arg: 0, flag: NULL, val: CHAR_MAX + 2 },
  option{ name: "delimiters", has_arg: 1, flag: NULL, val: 'e' as i32},
  option{ name: "struct-type", has_arg: 0, flag: NULL, val: 't' as i32},
  option{ name: "language", has_arg: 1, flag: NULL, val: 'L' as i32},
  option{ name: "slot-name", has_arg: 1, flag: NULL, val: 'K' as i32},
  option{ name: "initializer-suffix", has_arg: 1, flag: NULL, val: 'F' as i32},
  option{ name: "hash-fn-name", has_arg: 1, flag: NULL, val: 'H' as i32}, /* backward compatibility */
  option{ name: "hash-function-name", has_arg: 1, flag: NULL, val: 'H' as i32},
  option{ name: "lookup-fn-name", has_arg: 1, flag: NULL, val: 'N' as i32}, /* backward compatibility */
  option{ name: "lookup-function-name", has_arg: 1, flag: NULL, val: 'N' as i32},
  option{ name: "class-name", has_arg: 1, flag: NULL, val: 'Z' as i32},
  option{ name: "seven-bit", has_arg: 0, flag: NULL, val: '7' as i32},
  option{ name: "compare-strncmp", has_arg: 0, flag: NULL, val: 'c' as i32},
  option{ name: "readonly-tables", has_arg: 0, flag: NULL, val: 'C' as i32},
  option{ name: "enum", has_arg: 0, flag: NULL, val: 'E' as i32},
  option{ name: "includes", has_arg: 0, flag: NULL, val: 'I' as i32},
  option{ name: "global-table", has_arg: 0, flag: NULL, val: 'G' as i32},
  option{ name: "word-array-name", has_arg: 1, flag: NULL, val: 'W' as i32},
  option{ name: "length-table-name", has_arg: 1, flag: NULL, val: CHAR_MAX + 4 },
  option{ name: "switch", has_arg: 1, flag: NULL, val: 'S' as i32},
  option{ name: "omit-struct-type", has_arg: 0, flag: NULL, val: 'T' as i32},
  option{ name: "key-positions", has_arg: 1, flag: NULL, val: 'k' as i32},
  option{ name: "compare-strlen", has_arg: 0, flag: NULL, val: 'l' as i32}, /* backward compatibility */
  option{ name: "compare-lengths", has_arg: 0, flag: NULL, val: 'l' as i32},
  option{ name: "duplicates", has_arg: 0, flag: NULL, val: 'D' as i32},
  option{ name: "fast", has_arg: 1, flag: NULL, val: 'f' as i32},
  option{ name: "initial-asso", has_arg: 1, flag: NULL, val: 'i' as i32},
  option{ name: "jump", has_arg: 1, flag: NULL, val: 'j' as i32},
  option{ name: "multiple-iterations", has_arg: 1, flag: NULL, val: 'm' as i32},
  option{ name: "no-strlen", has_arg: 0, flag: NULL, val: 'n' as i32},
  option{ name: "occurrence-sort", has_arg: 0, flag: NULL, val: 'o' as i32},
  option{ name: "optimized-collision-resolution", has_arg: 0, flag: NULL, val: 'O' as i32},
  option{ name: "pic", has_arg: 0, flag: NULL, val: 'P' as i32},
  option{ name: "string-pool-name", has_arg: 1, flag: NULL, val: 'Q' as i32},
  option{ name: "null-strings", has_arg: 0, flag: NULL, val: CHAR_MAX + 3 },
  option{ name: "random", has_arg: 0, flag: NULL, val: 'r' as i32},
  option{ name: "size-multiple", has_arg: 1, flag: NULL, val: 's' as i32},
  option{ name: "help", has_arg: 0, flag: NULL, val: 'h' as i32},
  option{ name: "version", has_arg: 0, flag: NULL, val: 'v' as i32},
  option{ name: "debug", has_arg: 0, flag: NULL, val: 'd' as i32},
  option{ name: NULL, has_arg: 0, flag: NULL, val: 0 }
];






impl Options {
    
    /* Constructor.  */
    pub fn new(&mut self) -> Options {
        _option_word = C;
        _input_file_name = null_mut();
        _output_file_name = null_mut();
        _language = null_mut();
        _jump = DEFAULT_JUMP_VALUE;
        _initial_asso_value = 0;
        _asso_iterations = 0;
        _total_switches = 1;
        _size_multiple = 1;
        _function_name = DEFAULT_FUNCTION_NAME;
        _slot_name = DEFAULT_SLOT_NAME;
        _initializer_suffix = DEFAULT_INITIALIZER_SUFFIX;
        _class_name = DEFAULT_CLASS_NAME;
        _hash_name = DEFAULT_HASH_NAME;
        _wordlist_name = DEFAULT_WORDLIST_NAME;
        _lengthtable_name = DEFAULT_LENGTHTABLE_NAME;
        _stringpool_name = DEFAULT_STRINGPOOL_NAME;
        _delimiters = DEFAULT_DELIMITERS;
         
    }

    /* Parses the options given in the command-line arguments.  */   
    pub fn parse_options(&mut self, mut argc: i32, mut argv: &[*mut char]) {
        let mut option_char: i32;

        program_name = argv[0];
        _argument_count = argc;
        _argument_vector = argv;

        while ((option_char = getopt_long (_argument_count, _argument_vector,
            "acCdDe:Ef:F:gGhH:i:Ij:k:K:lL:m:nN:oOpPQ:rs:S:tTvW:Z:7",
            long_options, null_mut())) != -1) {
                match option_char {
                    'a' => {} /* Generated code uses the ANSI prototype format.  */
                    'c' =>  _option_word |= COMP, /* Generate strncmp rather than strcmp.  */
                    'C' => _option_word |= CONST,  /* Make the generated tables readonly (const).  */ 
                    'd' => {   /* Enable debugging option.  */
                        _option_word |= DEBUG;
                        eprint!("Starting program {}, version {}, with debugging on.\n",
                        program_name, version_string);
                    }
                    'D' => _option_word |= DUP, /* Enable duplicate option.  */
                    'e' => _delimiters = /*getopt*/optarg,
                    'E' =>  _option_word |= ENUM,
                    'f' => {}, /* Generate the hash table "fast".  */
                    'F' => _initializer_suffix = /*getopt*/optarg,
                    'g' => {}, /* Use the 'inline' keyword for generated sub-routines, ifdef __GNUC__.  */
                    'G' =>  _option_word |= GLOBAL, /* Make the keyword table a global variable.  */
                    'h' => long_usage(std::io::stdout()), /* Displays a list of helpful Options to the user.  */
                    'H' =>  _hash_name = /*getopt*/optarg, /* Sets the name for the hash function.  */
                    'i' => { /* Sets the initial value for the associated values array.  */
                        if((_initial_asso_value = atoi (/*getopt*/optarg)) < 0) {
                            eprint!("Initial value {} should be non-zero, ignoring and continuing.\n", _initial_asso_value);
                        }

                        if(option[RANDOM as i32]) {
                            eprint!("warning, -r option superceeds -i, ignoring -i option and continuing\n");
                        }
                    }
                    'I' => _option_word |= INCLUDE, /* Enable #include statements.  */
                    'j' => { /* Sets the jump value, must be odd for later algorithms.  */
                        if ((_jump = atoi (/*getopt*/optarg)) < 0)
                        {
                          eprint!("Jump value {} must be a positive number.\n", _jump);
                          short_usage(std::io::stdout());
                          std::process::exit(1);
                        } else if (_jump && ((_jump % 2) == 0)) {
                            eprint!("Jump value {} should be odd, adding 1 and continuing...\n", _jump);
                            _jump += 1;
                        }
                    }  
                    'k' => {
                        _option_word |= POSITIONS;
                        let BAD_VALUE: i32 = -3;
                        let EOS: i32 = PositionIterator::EOS;
                        let mut value: i32;
                        let sparser: PositionStringParser = PositionStringParser::new(/*getopt*/optarg, 1, Positions::MAX_KEY_POS, Positions::LASTCHAR, BAD_VALUE, EOS);
                        
                        if(/*getopt*/optarg [0] == '*') { /* Use all the characters for hashing!!!! */
                            _key_positions.set_useall(true);
                        } else {
                            _key_positions.set_useall(false);
                            let mut key_positions: *mut i32 = _key_positions.pointer();
                            let mut key_pos: *mut i32 = key_positions;

                            while ((value = sparser.nextPosition()) != EOS) {

                                if(value == BAD_VALUE) {
                                    eprint!("Invalid position value or range, use 1,2,3-{},'$' or '*'.\n",
                                         Positions::MAX_KEY_POS);
                                    short_usage(std::io::stdout());
                                    std::process::exit(1);
                                }

                                if((key_pos as usize - key_positions as usize) / 4 == Positions::MAX_SIZE) {

                                    /* More than Positions::MAX_SIZE key positions.
                                    Since all key positions are in the range
                                    0..Positions::MAX_KEY_POS-1 or == Positions::LASTCHAR,
                                    there must be duplicates.  */
                                    eprint!("Duplicate key positions selected\n");
                                    short_usage(std::io::stdout());
                                    std::process::exit(1);
                                }

                                if(value != Positions::LASTCHAR) {
                                    /* We use 0-based indices in the class Positions.  */
                                    value = value - 1;
                                }
                                *key_pos = value;
                                key_pos = key_pos.offset(1);
                            }

                            let mut total_keysig_size: u32 = (key_pos as usize - keypositions as usize) / 4;
                            if(total_keysig_size == 0) {
                                eprint!("No key positions selected.\n");
                                short_usage(std::io::stdout());
                                std::process::exit(1);
                            }

                            _key_positions.set_size (total_keysig_size);

                            /* Sorts the key positions *IN REVERSE ORDER!!*
                            This makes further routines more efficient.  Especially
                            when generating code.  */
                            if (!_key_positions.sort()) {
                                eprint!("Duplicate key positions selected\n");
                                short_usage(std::io::stdout());
                                std::process::exit(1);
                            }

                        }
                    
                    }

                    'K' => _slot_name = /*getopt*/optarg, /* Make this the keyname for the keyword component field.  */
                    'l' => _option_word |= LENTABLE, /* Create length table to avoid extra string compares.  */
                    'L' => {
                        /* Deal with different generated languages.  */
                        _language = null_mut();
                        set_language (/*getopt*/optarg);
                    }
                    'm' => {
                        /* Multiple iterations for finding good asso_values.  */
                        if ((_asso_iterations = atoi (/*getopt*/optarg)) < 0) {
                            eprint!("asso_iterations value must not be negative, assuming 0\n");
                            _asso_iterations = 0;
                        }
                    }
                    'n' => _option_word |= NOLENGTH,  /* Don't include the length when computing hash function.  */
                    'N' =>  _function_name = /*getopt*/optarg, /* Make generated lookup function name be optarg.  */
                    'o' => {} /* Order input by frequency of key set occurrence.  */
                    'O' => {} /* Optimized choice during collision resolution.  */
                    'p' => {} /* Generated lookup function a pointer instead of int.  */
                    'P' => _option_word |= SHAREDLIB, /* Optimize for position-independent code.  */
                    'Q' => _stringpool_name = /*getopt*/optarg, /* Sets the name for the string pool.  */
                    'r' => {
                        /* Utilize randomness to initialize the associated values table.  */
                        _option_word |= RANDOM;
                        if (_initial_asso_value != 0) {
                            eprint!("warning, -r option supersedes -i, disabling -i option and continuing\n");
                        }
                    }
                    's' => {
                        /* Range of associated values, determines size of final table.  */
                        
                        let mut numerator: f32;
                        let mut denominator: f32 = 1;
                        let mut invalid: bool = false;
                        let mut endptr: *mut char;

                        numerator = strtod(/*getopt*/optarg, &endptr);
                        if(endptr == /*getopt*/optarg) {
                            invalid = true;
                        }  else if (*endptr != '\0') {
                            if (*endptr == '/') {
                                char *denomptr = endptr.offset(1);
                                denominator = strtod (denomptr, &endptr);
                                if (endptr == denomptr || *endptr != '\0'){
                                    invalid = true;
                                }
                            } else {
                                invalid = true;
                            } 
                        }
                        if (invalid) {
                            eprint!("Invalid value for option -s.\n");
                            short_usage(std::io::stderr());
                            std::process::exit(1);
                        }

                        _size_multiple = numerator / denominator;
                        /* Backward compatibility: -3 means 1/3.  */
                        if (_size_multiple < 0){
                            _size_multiple = 1 / (-_size_multiple);
                        }
                        /* Catch stupid users.  */
                        if (_size_multiple == 0) {
                            _size_multiple = 1;
                        }
                        /* Warnings.  */
                        if (_size_multiple > 50) {
                            eprint!("Size multiple {} is excessive, did you really mean this?! (try '{} --help' for help)\n", _size_multiple, program_name);
                        }
                        
                        else if (_size_multiple < 0.01f) {
                            eprint!("Size multiple {} is extremely small, did you really mean this?! (try '{} --help' for help)\n", _size_multiple, program_name);
                        }
                    }

                    'S' => {
                        _option_word |= SWITCH;
                        _total_switches = atoi (/*getopt*/optarg);
                        if (_total_switches <= 0) {
                            eprint!("number of switches {} must be a positive number\n", /*getopt*/optarg);
                            short_usage(std::io::stderr());
                            std::process::exit(1);
                        }
                    }
                    't' => _option_word |= OptionType::TYPE as i32, /* Enable the TYPE mode, allowing arbitrary user structures.  */
                    'T' =>  _option_word |= NOTYPE, /* Don't print structure definition.  */
                    'v' => {
                        print!("GNU gperf {}\n", version_string);
                        print!("Copyright (C) {} Free Software Foundation, Inc.\n\
                                License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>\n\
                                This is free software: you are free to change and redistribute it.\n\
                                There is NO WARRANTY, to the extent permitted by law.\n\
                                        ",
                                "1989-1998, 2000-2004, 2006-2009");
                        print!("Written by {} and {}.\n",
                                "Douglas C. Schmidt", "Bruno Haible");
                        std::process::exit(0);
                    }
                    'W' =>  _wordlist_name = /*getopt*/optarg, /* Sets the name for the hash table array.  */
                    'Z' =>  _class_name = /*getopt*/optarg, /* Set the class name.  */
                    '7' =>   _option_word |= SEVENBIT,/* Assume 7-bit characters.  */
                    from_u32(128) => _output_file_name = /*getopt*/optarg, /* Set the output file name.  */
                    from_u32(129) => _option_word |= OptionType::UPPERLOWER as i32, /* Case insignificant.  */
                    from_u32(130) => _option_word |= NULLSTRINGS, /* Use NULL instead of "".  */
                    from_u32(131) => _lengthtable_name = /*getopt*/optarg, /* Sets the name for the length table array.  */
                    _ => {
                        short_usage(std::io::stderr());
                        std::process::exit(1);
                    }
                    
                }

                if (/*getopt*/optind < argc){
                    _input_file_name = argv[/*getopt*/optind];
                    optind += 1;
                }
    
                if (/*getopt*/optind < argc) {
                    eprint!("Extra trailing arguments to {}.\n", program_name);
                    short_usage(std::io::stderr());
                    std::process::exit(1);
                }

            }
    }

    /* Prints the given options.  */
    pub const fn print_options(&mut self) {
        print!("/* Command-line: ");

        let mut i: i32 = 0;
        while (i < _argument_count) {

            let mut arg: *const char = *(_argument_vector.offset(i));

            /* Escape arg if it contains shell metacharacters.  */
            if (*arg == '-') {

                print!("{}", *arg);
                arg = arg.offset(1);

                if(*arg >= 'A' && *arg <= 'Z' || *arg >= 'a' && *arg <= 'z') {
                    print!("{}", *arg);
                    arg = arg.offset(1);
                } else if(*arg == '-') {
                    loop {
                        print!("{}", *arg);
                        arg = arg.offset(1);

                        if(*arg >= 'A' && *arg <= 'Z' || *arg >= 'a' && *arg <= 'z' || *arg == '-') {
                            break;
                        }
                    }

                    if(*arg == '=') {
                        print!("{}", *arg);
                        arg = arg.offset(1);
                    }
                } 
            }

            if(strpbrk (arg, "\t\n !\"#$&'()*;<>?[\\]`{|}~") != null_mut()) {
                if(strchr(arg, '\'') != null_mut()) {
                    print!("\"");
                    while (*arg != '\0') {
                        if (*arg == '\"' || *arg == '\\' || *arg == '$' || *arg == '`') {
                            print!("\\");
                        }
                        print!("\"");
                        arg = arg.offset(1);
                    }

                    print!("\"");
                } else {
                    print!("\'");
                    while (*arg) {
                        if (*arg == '\\') {
                            print!("\\");
                        }
                        print!("{}", *arg);
                        arg = arg.offset(1);
                    }
                    print!("\'");
                }
 
            } else {
                print!("{}", arg);
            }

            print!(" ");

            i += 1;
        }

        print!(" */");
    }

    /* Accessors.  */

    
    /* Sets a given boolean option.  */
    #[inline]
    pub fn set(&mut self, mut option: OptionType) {
        _option_word |= option as i32;
    }

    /* Returns the input file name.  */
    #[inline]
    pub const fn get_input_file_name(&mut self) -> *const char {
        return _input_file_name;
    }

    /* Returns the output file name.  */
    #[inline]
    pub const fn get_output_file_name(&mut self) -> *const char {
        return output_file_name;
    }

    /* Sets the output language, if not already set.  */
    pub fn set_language(&mut self, mut language: *const char) {
        if(_language == null_mut()) {
            _language = language;
            _option_word &= !(OptionType::KRC as i32 |OptionType::C as i32 | OptionType::ANSIC as i32 
                    | OptionType::CPLUSPLUS as i32);
            if (!strcmp (language, "KR-C")) {
                _option_word |= OptionType::KRC as i32;
            }
                
            else if (!strcmp (language, "C")) {
                _option_word |= OptionType::C as i32;
            }
                
            else if (!strcmp (language, "ANSI-C")) {
                _option_word |= OptionType::ANSIC as i32;
            }
                
            
            else if (!strcmp (language, "C++")) {
                _option_word |= OptionType::CPLUSPLUS as i32;
            }
                
            else {
                eprintln!("unsupported language option {}, defaulting to C\n",
                        language);
                _option_word |= OptionType::C as i32;
            }
        }
    }

    /* Returns the jump value.  */
    #[inline]
    pub const fn get_jump(&mut self) -> i32 {
        return _jump;
    } 

    /* Returns the initial associated character value.  */
    #[inline]
    pub const fn get_initial_asso_value(&mut self) -> i32 {
        return _initial_asso_value;
    }

    /* Returns the number of iterations for finding good asso_values.  */
    #[inline]
    pub const fn get_asso_iterations(&mut self) -> i32 {
        return _asso_iterations;
    }

    /* Returns the total number of switch statements to generate.  */
    #[inline]
    pub const fn get_total_switches(&mut self) -> i32 {
        return _total_switches;
    }

    /* Sets the total number of switch statements, if not already set.  */
    pub fn set_total_switches(&mut self, mut total_switches: i32) {
        if (!(_option_word & SWITCH))
        {
            _option_word |= SWITCH;
            _total_switches = total_switches;
        }
    }

    /* Returns the factor by which to multiply the generated table's size.  */
    #[inline]
    pub const fn get_size_multiple(&mut self) -> f32 {
        return _size_multiple;
    }

    /* Returns the generated function name.  */
    #[inline]
    pub const fn get_function_name(&mut self) -> *const char {
        return _function_name;
    }

    /* Sets the generated function name, if not already set.  */
    pub fn set_function_name(&mut self, mut name: *const char) {
        
        if (_function_name == DEFAULT_FUNCTION_NAME) {
            _function_name = name;
        }
            
    }

    /* Returns the keyword key name.  */
    #[inline]
    pub const fn get_slot_name(&mut self) -> *const char {
        return _slot_name;
    }

    /* Sets the keyword key name, if not already set.  */
    pub const fn set_slot_name(&mut self, mut name: *const char) {
        
        if (_slot_name == DEFAULT_SLOT_NAME) {
            _slot_name = name;
        }
        
    }

    /* Returns the struct initializer suffix.  */
    #[inline]
    pub const fn get_initializer_suffix(&mut self) -> *const char {
        return _initializer_suffix;
    } 

    /* Sets the struct initializer suffix, if not already set.  */
    pub fn set_initializer_suffix(&mut self, mut initializers: *const char) {
        
        if (_initializer_suffix == DEFAULT_INITIALIZER_SUFFIX){
            _initializer_suffix = initializers;
        }
            
    }

    /* Returns the generated class name.  */
    #[inline]
    pub const fn get_class_name(&mut self) -> *const char {
        return _class_name;
    }

    /* Sets the generated class name, if not already set.  */
    pub fn set_class_name(&mut self, mut name: *const char) {

        if (_class_name == DEFAULT_CLASS_NAME) {
            _class_name = name;
        }

    }

    /* Returns the hash function name.  */
    #[inline]
    pub const fn get_hash_name(&mut self) -> *const char {
        return _hash_name;
    }

    /* Sets the hash function name, if not already set.  */
    pub fn set_hash_name(&mut self, mut name: *const char) {
        
        if (_hash_name == DEFAULT_HASH_NAME) {
            _hash_name = name;
        }
    }

    /* Returns the hash table array name.  */
    #[inline]
    pub const fn get_wordlist_name(&mut self) -> *const char {
        return _wordlist_name;
    }

    /* Sets the hash table array name, if not already set.  */
    pub fn set_wordlist_name(&mut self, mut name: *const char) {
        
        if (_wordlist_name == DEFAULT_WORDLIST_NAME) {
            _wordlist_name = name;
        }
    
    }

    /* Returns the length table array name.  */
    pub const fn get_lengthable_name(&mut self) -> *const char {
        return _lengthable_name;
    }

    /* Sets the length table array name, if not already set.  */
    pub fn set_lengthable_name(&mut self, mut name: *const char) {

        if (_lengthtable_name == DEFAULT_LENGTHTABLE_NAME) {
            lengthtable_name = name;
        }

    }

    /* Returns the string pool name.  */
    #[inline]
    pub const fn get_stringpool_name(&mut self) -> *const char {
        return _stringpool_name;
    }

    /* Sets the string pool name, if not already set.  */
    pub fn set_stringpool_name(&mut self, mut name: *const char) {
        
        if (_stringpool_name == DEFAULT_STRINGPOOL_NAME) {
            _stringpool_name = name;
        }
    }

    /* Returns the string used to delimit keywords from other attributes.  */
    #[inline]
    pub const fn get_delimiters(&mut self) -> *const char {
        return _delimiters;
    }

    /* Sets the delimiters string, if not already set.  */
    pub fn set_delimiters(&mut self, mut delimiters: *const char) {

        if (_delimiters == DEFAULT_DELIMITERS) {
            _delimiters = delimiters;
        }
                
    }

    
    /* Returns key positions.  */
    #[inline]
    pub const fn get_key_positions(&mut self) -> &Positions {
        return _key_positions;
    }

    /* Prints program usage to given stream.  */
    fn short_usage(mut stream: OpenOptions) {

        writeln!(stream, "Try '{} --help' for more information.\n", program_name);

    }

    /* Prints program usage to given stream.  */
    fn long_usage(mut stream: FILE) {

        writeln!(stream, "GNU 'gperf' generates perfect hash functions.");
        writeln!(stream, "");
        writeln!(stream, "Usage: {} [OPTION]... [INPUT-FILE]", program_name);
        writeln!(stream, "");
        writeln!(stream, "If a long option shows an argument as mandatory, then it is mandatory\n for the equivalent short option also.");
        writeln!(stream, "");
        writeln!(stream, "Output file location:");
        writeln!(stream, "      --output-file=FILE Write output to specified file.");
        writeln!(stream, "The results are written to standard output if no output file is specified\n or if it is -.");
        writeln!(stream, "");
        writeln!(stream, "Input file interpretation:");
        writeln!(stream, "  -e, --delimiters=DELIMITER-LIST\n                         Allow user to provide a string containing delimiters\n                         used to separate keywords from their attributes.\n                         Default is \",\".\"");
        writeln!(stream, "  -t, --struct-type      Allows the user to include a structured type\n                         declaration for generated code. Any text before %%%%\n                         is considered part of the type declaration. Key\n                         words and additional fields may follow this, one\n                         group of fields per line.");    
        writeln!(stream, "      --ignore-case      Consider upper and lower case ASCII characters as\n                         equivalent. Note that locale dependent case mappings\n                         are ignored.");
        writeln!(stream, "");
        writeln!(stream, "Language for the output code:");
        writeln!(stream, "  -L, --language=LANGUAGE-NAME\n                         Generates code in the specified language. Languages\n                         handled are currently C++, ANSI-C, C, and KR-C. The                         default is C.");
        writeln!(stream, "");
        writeln!(stream, "Details in the output code:");
        writeln!("  -K, --slot-name=NAME   Select name of the keyword component in the keyword\n                         structure.");
        writeln!(stream, "  -F, --initializer-suffix=INITIALIZERS\n                         Initializers for additional components in the keyword\n                         structure.");
        writeln!(stream, "  -H, --hash-function-name=NAME\n                         Specify name of generated hash function. Default is\n                         'hash'.");
        writeln!(stream, "  -N, --lookup-function-name=NAME\n                         Specify name of generated lookup function. Default\n                         name is 'in_word_set'.");
        writeln!(stream, "  -Z, --class-name=NAME  Specify name of generated C++ class. Default name is\n                         'Perfect_Hash'.");
        writeln!(stream, "  -7, --seven-bit        Assume 7-bit characters.\n");
        writeln!(stream, "  -l, --compare-lengths  Compare key lengths before trying a string\n                         comparison. This is necessary if the keywords\n                         contain NUL bytes. It also helps cut down on the\n                         number of string comparisons made during the lookup.\n");
        writeln!(stream, "  -c, --compare-strncmp  Generate comparison code using strncmp rather than\n                         strcmp.");
        writeln!(stream, "  -C, --readonly-tables  Make the contents of generated lookup tables\n                         constant, i.e., readonly.");
        writeln!(stream, "  -E, --enum             Define constant values using an enum local to the\n                         lookup function rather than with defines.");
        writeln!(stream, "  -I, --includes         Include the necessary system include file <string.h>\n                         at the beginning of the code.");
        writeln!(stream, "  -G, --global-table     Generate the static table of keywords as a static\n                         global variable, rather than hiding it inside of the\n                         lookup function (which is the default behavior).");
        writeln!(stream, "  -P, --pic              Optimize the generated table for inclusion in shared\n                         libraries.  This reduces the startup time of programs\n                         using a shared library containing the generated code.");
        writeln!(stream, "  -Q, --string-pool-name=NAME\n                         Specify name of string pool generated by option --pic.\n                         Default name is 'stringpool'.");
        writeln!(stream, "      --null-strings     Use NULL strings instead of empty strings for empty\n                         keyword table entries.");
        writeln!(stream, "  -W, --word-array-name=NAME\n                         Specify name of word list array. Default name is\n                         'wordlist'.\n");
        writeln!(stream, "  -S, --switch=COUNT     Causes the generated C code to use a switch\n                         statement scheme, rather than an array lookup table.\n                         This can lead to a reduction in both time and space\n                         requirements for some keyfiles. The COUNT argument\n                         determines how many switch statements are generated.\n                         A value of 1 generates 1 switch containing all the\n                         elements, a value of 2 generates 2 tables with 1/2\n                         the elements in each table, etc. If COUNT is very\n                         large, say 1000000, the generated C code does a\n                         binary search.\n");
        writeln!(stream, "  -T, --omit-struct-type\n                         Prevents the transfer of the type declaration to the\n                         output file. Use this option if the type is already\n                         defined elsewhere.\n");
        writeln!(stream, "");
        writeln!(stream, "Algorithm employed by gperf:");
        writeln!(stream, "  -k, --key-positions=KEYS\n                         Select the key positions used in the hash function.\n                         The allowable choices range between 1-{}, inclusive.\n                         The positions are separated by commas, ranges may be\n                         used, and key positions may occur in any order.\n                         Also, the meta-character '*' causes the generated\n                         hash function to consider ALL key positions, and $\n                         indicates the \"final character\" of a key, e.g.,\n                         $,1,2,4,6-10.\n", Positions::MAX_KEY_POS);
        writeln!(stream, "  -D, --duplicates       Handle keywords that hash to duplicate values. This\n                         is useful for certain highly redundant keyword sets.");
        writeln!(stream, "  -m, --multiple-iterations=ITERATIONS\n                         Perform multiple choices of the -i and -j values,\n                         and choose the best results. This increases the\n                         running time by a factor of ITERATIONS but does a\n                         good job minimizing the generated table size.\n");
        writeln!(stream, "  -i, --initial-asso=N   Provide an initial value for the associate values\n                         array. Default is 0. Setting this value larger helps\n                         inflate the size of the final table.\n");
        writeln!(stream, "  -j, --jump=JUMP-VALUE  Affects the \"jump value\", i.e., how far to advance\n                         the associated character value upon collisions. Must\n                         be an odd number, default is {}.", DEFAULT_JUMP_VALUE);
        writeln!(stream, "  -n, --no-strlen        Do not include the length of the keyword when\n                         computing the hash function.");
        writeln!(stream, "  -r, --random           Utilizes randomness to initialize the associated\n                         values table.");
        writeln!(stream, "  -s, --size-multiple=N  Affects the size of the generated hash table. The\n                         numeric argument N indicates \"how many times larger\n                         or smaller\" the associated value range should be,\n                         in relationship to the number of keys, e.g. a value\n                         of 3 means \"allow the maximum associated value to\n                         be about 3 times larger than the number of input\n                         keys\". Conversely, a value of 1/3 means \"make the\n                         maximum associated value about 3 times smaller than\n                         the number of input keys\". A larger table should\n                         decrease the time required for an unsuccessful\n                         search, at the expense of extra table space. Default\n                         value is 1.");
        writeln!(stream, "");
        writeln!(stream, "Informative output:\n  -h, --help             Print this message.\n  -v, --version          Print the gperf version number.\n  -d, --debug            Enables the debugging option (produces verbose\n                         output to the standard error)." );
        writeln!(stream, "");
        writeln!(stream, "Report bugs to <bug-gnu-gperf@gnu.org>.");
    }

}

/* Tests a given boolean option.  Returns true if set, false otherwise.  */
impl IndexMut<OptionType> for Options {
    #[inline]
    fn index_mut(&mut self, option: OptionType) -> bool {
        return _option_word & (option as i32);
    }
}

/* Dumps option status when debugging is enabled.  */
impl Drop for Options {
    fn drop(&mut self) {
        if(_option_word & OptionType::DEBUG as i32) {
            eprintln!("\ndumping Options:
\nTYPE is........: {}
\nUPPERLOWER is..: {}
\nKRC is.........: {}
\nC is...........: {}
\nANSIC is.......: {}
\nCPLUSPLUS is...: {}
\nSEVENBIT is....: {}
\nLENTABLE is....: {}
\nCOMP is........: {}
\nCONST is.......: {}
\nENUM is........: {}
\nINCLUDE is.....: {}
\nGLOBAL is......: {}
\nNULLSTRINGS is.: {}
\nSHAREDLIB is...: {}
\nSWITCH is......: {}
\nNOTYPE is......: {}
\nDUP is.........: {}
\nNOLENGTH is....: {}
\nRANDOM is......: {}
\nDEBUG is.......: {}
\nlookup function name = {} 
\nhash function name = {}
\nword list name = {}
\nlength table name = {}
\nstring pool name = {}
\nslot name = {}
\ninitializer suffix = {}
\nasso_values iterations = {}
\njump value = {}
\nhash table size multiplier = {}
\ninitial associated value = {}
\ndelimiters = {}
\nnumber of switch statements = {}",
            _option_word & OptionType::TYPE as i32 ? "enabled" : "disabled",
            _option_word & OptionType::UPPERLOWER ? "enabled" : "disabled",
            _option_word & OptionType::KRC as i32 ? "enabled" : "disabled",
            _option_word & OptionType::C as i32 ? "enabled" : "disabled",
            _option_word & OptionType::ANSIC as i32 ? "enabled" : "disabled",
            _option_word & OptionType::CPLUSPLUS as i32 ? "enabled" : "disabled",
            _option_word & OptionType::SEVENBIT as i32 ? "enabled" : "disabled",
            _option_word & OptionType::LENTABLE as i32 ? "enabled" : "disabled",
            _option_word & OptionType::COMP as i32 ? "enabled" : "disabled",
            _option_word & OptionType::CONST as i32 ? "enabled" : "disabled",
            _option_word & OptionType::ENUM as i32 ? "enabled" : "disabled",
            _option_word & OptionType::INCLUDE as i32 ? "enabled" : "disabled",
            _option_word & OptionType::GLOBAL as i32 ? "enabled" : "disabled",
            _option_word & OptionType::NULLSTRINGS as i32 ? "enabled" : "disabled",
            _option_word & OptionType::SHAREDLIB as i32 ? "enabled" : "disabled",
            _option_word & OptionType::SWITCH as i32 ? "enabled" : "disabled",
            _option_word & OptionType::NOTYPE as i32 ? "enabled" : "disabled",
            _option_word & OptionType::DUP as i32 ? "enabled" : "disabled",
            _option_word & OptionType::NOLENGTH as i32 ? "enabled" : "disabled",
            _option_word & OptionType::RANDOM as i32 ? "enabled" : "disabled",
            _option_word & OptionType::DEBUG as i32 ? "enabled" : "disabled",
            _function_name, _hash_name, _wordlist_name, _lengthtable_name,
            _stringpool_name, _slot_name, _initializer_suffix,
            _asso_iterations, _jump, _size_multiple, _initial_asso_value,
            _delimiters, _total_switches);

            if (_key_positions.is_useall()) {
                eprintln!("all characters are used ni the hash function");
            } else {
                eprintln!("maximum keysig = {}\nkey positions are : ", _key_positions.get_size());

                let mut iter: PositionIterator = _key_positions.iterator();
                let mut pos: i32;
                while ((pos = iter.next()) != PositonsIterator::EOS) {
                    if (pos == Positions::LASTCHAR) {
                        eprintln!("$");
                    } else {
                        eprintln!("{}", pos + 1);
                    }
                }
            }

            eprintln!("finished dumping Options\n");
        }
    }
}

struct PositionStringParser {

    /* A pointer to the string provided by the user.  */
    _str: *const char,
    
    /* Smallest possible value, inclusive.  */
    _low_bound: i32,

    /* Greatest possible value, inclusive.  */
    _high_bound: i32,

    /* A value marking the abstract "end of word" ( usually '$').  */
    _end_word_marker: i32,

    /* Error value returned when input is syntactically erroneous.  */
    _error_value: i32,

    /* Value returned after last key is processed.  */
    _end_marker: i32,

    /* Intermediate state for producing a range of positions.  */
    _in_range: bool, /* True while producing a range of positions.  */
    _range_upper_bound: i32, /* Upper bound (inclusive) of the range.  */
    _range_curr_value: i32 /* Last value returned.  */

}


impl PositionStringParser {

    /* Initializes a key position strng parser for string STR.  */
    pub fn new(&mut self, s: *const char, 
                low_bound: i32, high_bound: i32, 
                end_word_marker: i32, error_value: i32, end_marker: i32) -> PositionStringParser {

        _str = s;
        _low_bound = low_bound;
        _high_bound = high_bound;
        _end_word_marker = end_word_marker;
        _error_value = error_value;
        _end_marker = end_marker;
        _in_range = false;
    }

    /* Returns the next key position from the given string.  */
    pub fn nextPosition() -> i32 {

        
        if(_in_range) {
            /* We are inside a range.  Return the next value from the range.  */
            _range_curr_value += 1;
            if(_range_curr_value >= _range_upper_bound) {
                _in_range = false;
            }
            return _range_curr_values;
        } else {
            /* Continue parsing the given string.  */
            while(*_str) {
                match *_str {

                    ',' => 
                        /* Skip the comma. */
                        _str += 1,

                    '$' => 
                    /* Valid key position.  */ 
                    { 
                        
                        _str += 1;
                        return _end_work_marker;
                    }

                    '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => 
                    /* Valid key position.  */
                    {
                        let curr_value: i32 = 0;
                        while(*_str as u8 >= '0' && *_str as u8 <= '9') {

                            curr_value = curr_value * 10 + (*_str - '0');
                            _str = _str.offset(1);
                        }
                        
                        if(*_str == '-') {
                            _str = _str.offset(1);
                            /* Starting a range of key positions.  */
                            _in_range = true;

                            _range_upper_bound = 0;
                            while (*_str as u8 >= '0' && *_str as u8 <= '9') {
                                _range_upper_bound = _range_upper_bound * 10 + (*_str - '0');
                                _str = _str.offset(1);
                            }

                            /* Verify range's upper bound.  */
                            if (!(_range_upper_bound > curr_value && _range_upper_bound <= _high_bound)) {
                                return _error_value;
                            }
                            _range_curr_value = curr_value;
                        }

                        /* Verify range's lower bound.  */
                        if (!(curr_value >= _low_bound && curr_value <= _high_bound)) {
                            return _error_value;
                        }
                        return curr_value;
                    }
  
                    _ => /*Invalid syntax. */
                        return error_value,


                }

            }
            return _end_marker;
        }

    }

}