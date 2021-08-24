import subprocess
import os

def install_cargo():
    curl = subprocess.Popen(('curl', 'https://sh.rustup.rs', '-sSf'), stdout=subprocess.PIPE)
    output = subprocess.check_output(('sh', '-s', '--', '-y'), stdin=curl.stdout)
    curl.wait()

def install_cargo_fuzz():
    subprocess.check_call(['cargo', 'install', 'cargo-fuzz'])

def run_fuzz():
    env = os.environ.copy()
    env["RUSTC_BOOTSTRAP"] = "1"
    subprocess.check_call(['cargo', 'fuzz', 'run', 'runtime-fuzzer',
                           '--', '-len_control=0' '-prefer_small=0'],
                          env=env, cwd='../test-utils/runtime-tester/fuzz')

def main():
    install_cargo()
    install_cargo_fuzz()
    run_fuzz()

if __name__ == "__main__":
    main()