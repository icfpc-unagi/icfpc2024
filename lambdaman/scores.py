import subprocess
import re

def get_scores():
    # Execute the cargo command to get scores and capture the standard error output
    result = subprocess.run(['cargo', 'run', '--bin', 'chat'], input='get lambdaman', text=True, capture_output=True, shell=False)
    return result.stderr

def get_ranks():
    # Execute the cargo command to get ranks and capture the standard error output
    result = subprocess.run(['cargo', 'run', '--bin', 'chat'], input='get scoreboard lambdaman', text=True, capture_output=True, shell=False)
    print(result)
    rank_lines = [line for line in result.stderr.split('\n') if 'Unagi' in line]
    print(rank_lines)
    assert(len(rank_lines) == 1)
    return rank_lines[0]

def parse_scores(score_text):
    # Split the input text by lines
    lines = score_text.strip().split('\n')
    
    # Define a regular expression to match the score pattern
    pattern = re.compile(r'\* \[(lambdaman\d+)\] Your score: (\d+)\. Best score: (\d+)\.')
    
    # List to hold the parsed scores
    scores = []
    
    # Process each line
    for line in lines:
        match = pattern.match(line)
        if match:
            problem_number = match.group(1)
            your_score = match.group(2)
            best_score = match.group(3)
            # Append the parsed result as a tuple
            scores.append((problem_number, your_score, best_score))
    
    return scores

def parse_ranks(rank_text):
    # Extract the ranks for Unagi
    ranks = [rank.strip() for rank in rank_text.replace("*", "").split('|') if rank.strip()]
    assert ranks[1] == "Unagi"
    return ranks[2:]

def combine_scores_and_ranks(scores, ranks):
    # Combine the parsed scores and ranks
    results = []
    for i, (problem_number, your_score, best_score) in enumerate(scores):
        if i < len(ranks):
            rank = ranks[i]
            results.append(f'{problem_number}\t{your_score}\t{best_score}\t{rank}')
    
    # Join all results into a single string separated by newlines
    output = '\n'.join(results)
    return output

def main():
    # Get and parse scores
    score_text = get_scores()
    scores = parse_scores(score_text)
    
    # Get and parse ranks
    rank_text = get_ranks()
    ranks = parse_ranks(rank_text)
    
    # Combine scores and ranks
    output = combine_scores_and_ranks(scores, ranks)
    
    # Print the output
    print(output)

if __name__ == '__main__':
    main()
