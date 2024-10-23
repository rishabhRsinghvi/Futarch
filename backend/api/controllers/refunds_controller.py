from flask import Blueprint, jsonify
from services.near_interaction import process_refund  # Adjust as needed

app = Blueprint('refunds', __name__)

@app.route('/projects/<int:project_id>/refund', methods=['POST'])
def process_refund_route(project_id):
    process_refund(project_id)
    return jsonify({"status": "refund initiated"}), 200
