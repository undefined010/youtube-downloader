from pytube import YouTube
import sys

def download_yt_video(url : str) -> None:

    YouTube(url).streams.first().download()

if __name__ == '__main__':

    if sys.argv[1].__len__ == 0 :
        sys.stderr("no link")

    download_yt_video(sys.argv[1])