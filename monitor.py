from datetime import datetime
from subprocess import Popen, TimeoutExpired

PROCESS_TIMEOUT = 30 * 60

def main():
	while True:
		print(str(datetime.now()))
		bot_process = Popen('cargo run', shell=True)
		try:
			bot_process.wait(timeout=PROCESS_TIMEOUT)
		except TimeoutExpired:
			pass
		finally:
			bot_process.kill()

if __name__ == '__main__':
	main()
