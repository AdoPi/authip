curl -X POST -d '{"ipv4": "172.0.1.3", "desc": "local1"}' -H "Content-Type:application/json"  http://localhost:8000/api/ips
curl -X POST -d '{"ipv4": "222.0.1.2", "desc": "local2"}' -H "Content-Type:application/json"  http://localhost:8000/api/ips
curl -X POST -d '{"ipv4": "333.0.1.2", "desc": "local2"}' -H "Content-Type:application/json"  http://localhost:8000/api/ips
curl -X POST -d '{"ipv4": "444.0.1.2", "desc": "local2"}' -H "Content-Type:application/json"  http://localhost:8000/api/ips
curl -X POST -d '{"ipv4": "555.0.1.2", "desc": "local2"}' -H "Content-Type:application/json"  http://localhost:8000/api/ips
# uniqueness
curl -X POST -d '{"ipv4": "172.0.1.2", "desc": "local2"}' -H "Content-Type:application/json"  http://localhost:8000/api/ips

echo ""
curl -X GET http://localhost:8000/api/ips
echo "TXT"
curl -X GET http://localhost:8000/api/ips.txt
curl -X GET http://localhost:8000/api/ips.txt -o ip.txt
echo ""
cat ip.txt
