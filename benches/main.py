import subprocess
import time
from tqdm import tqdm


if __name__ == "__main__":

    dirpath = "tests/data/linux-5-15-rc4"

    cmd1 = ["target/release/locate", ".txt", "-d", dirpath]
    cmd2 = ["find", dirpath, "-name", "*.txt"]
    cmd3 = ["target/release/locate", "", "-d", dirpath]
    cmd4 = ["find", dirpath, "-name", "*"]


    n = 100

    for cmd in [cmd1, cmd2, cmd3, cmd4]:
        t = 0.0
        
        for i in tqdm(range(n)):
            t0 = time.time()
            p = subprocess.Popen(cmd, stdout=subprocess.DEVNULL)
            p.wait()
            t += (time.time() - t0)

        print(t / n)
