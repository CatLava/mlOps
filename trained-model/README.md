run with
docker build --build-arg VERSION=AutoML_287f444c -t flask-predict .

$ docker run -p 5000:5000 -d --name flask-predict flask-predict
