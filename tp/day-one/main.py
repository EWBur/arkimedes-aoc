import pandas as pd
from pathlib import Path

day_one_dir = Path(__file__).resolve().parent

if __name__ == '__main__':
    df = pd.read_csv(day_one_dir / 'input.txt', sep=r'\s+', names = ['list1', 'list2'])
    df = df.apply(lambda col: col.sort_values().reset_index(drop=True))
    print(f'The total difference is: {(df['list2'] - df['list1']).abs().sum()}')