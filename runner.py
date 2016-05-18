import argparse
import os
import shlex
import subprocess
import sys

# Tell the runner how to run each languages example
LANGUAGES = {
    'rust': {
        'directory': 'rust/',
        'command': 'cargo run --bin {0:04d}',
        'list_completed': lambda: sorted([int(file[:-3]) for file in os.listdir('rust/src/bin/')]), 
    },
}

# Load the answers
ANSWERS = [int(line.rstrip('\n')) for line in open('problems/answers.txt').readlines()]

# Run an example
def run(language, number):
    info = LANGUAGES[language]

    # Change to the languages directory
    os.chdir(info['directory']);

    # Run the command
    command = shlex.split(info['command'].format(number))
    process = subprocess.Popen(command, stdout=subprocess.PIPE);

    # Print the output keeping track of the last line
    lastline = ''
    for line in process.stdout:
        line = line.decode('ascii')
        lastline = line
        sys.stdout.write(line)

    return int(lastline)

# Parse our args
parser = argparse.ArgumentParser(description='Process some integers.')
parser.add_argument(
    'language', 
    type=str, 
    choices=LANGUAGES.keys(),
    help='which language to run'
)
parser.add_argument(
    'number', 
    type=int, 
    help='which problem to run'
)
args = parser.parse_args()

# Make sure the problem has been completed
completed_problems = LANGUAGES[args.language]['list_completed']()
if not args.number in completed_problems:
    print('Problem {} has not been completed for language {}'
          .format(args.number, args.language))
    sys.exit(1)

# Run the problem
pre_run_message = '---- Running {} problem {} ----'.format(
    args.language, args.number)
print(pre_run_message)
answer = run(args.language, args.number)
print('-' * len(pre_run_message))

# Check to see if the answer is correct
if ANSWERS[args.number - 1] == -1:
    print('No answer for problem {} is saved'.format(args.number))
    sys.exit(1)
elif ANSWERS[args.number - 1] != answer:
    print('{}{} is the incorrect answer for problem {}{}'
          .format('\x1b[0;31m', answer, args.number, '\x1b[0m'))
    sys.exit(1)
elif ANSWERS[args.number - 1] == answer:
    print('{}{} is the correct answer for problem {}{}'
          .format('\x1b[0;32m', answer, args.number, '\x1b[0m'))
    sys.exit(0)
