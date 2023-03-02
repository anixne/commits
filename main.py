import os

# Create tons of files.

for i in range(100):
    os.system(f"touch ./files/{i}.txt")
    os.system(f"git add ./files/{i}.txt")
    os.system(f'git commit -m "{i}.txt"')

os.system("git push origin main")