import os
from flask import request, jsonify, Flask

app = Flask(__name__)

@app.route('/', methods=['GET', 'POST'])
def is_prime():
    if request.method == 'POST':
        return jsonify({"is_prime": prime(request.json["number"])})
    else:
        raise Exception("Method not allowed")

def prime(number: int) -> bool:
    if number < 2:
        return False
    for i in range(2, int(number ** 0.5) + 1):
        if number % i == 0:
            return False
    return True

if __name__ == '__main__':
    # Configuração baseada em variáveis de ambiente
    debug_mode = os.environ.get('FLASK_DEBUG', 'False').lower() == 'true'
    port = int(os.environ.get('PORT', 3000))
    host = os.environ.get('HOST', '127.0.0.1')
    
    app.run(debug=debug_mode, host=host, port=port)