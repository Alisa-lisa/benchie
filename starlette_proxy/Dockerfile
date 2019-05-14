FROM python:3.6.5-stretch

WORKDIR /usr/src/app

RUN  apt-get update

COPY requirements.txt ./
COPY app.py ./

RUN pip install --upgrade pip
RUN pip install -r requirements.txt  --upgrade

EXPOSE 5003

CMD ["python", "app.py"]