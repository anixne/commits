import os

# Create tons of files.

NUMBER_OF_COMMITS = 500

for i in range(NUMBER_OF_COMMITS):
    os.system(f"touch ./files/{i}.txt")
    os.system(f"git add ./files/{i}.txt")
    os.system(f'git commit -m "{i}.txt"')

os.system("git push origin main")