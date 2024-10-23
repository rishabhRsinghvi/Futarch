from flask import Blueprint, request, jsonify
from services.near_interaction import create_project, fund_project

app = Blueprint('projects', __name__)

@app.route('/projects', methods=['POST'])
def create_new_project():
    data = request.json
    project_id = create_project(data['owner'])
    return jsonify({"project_id": project_id}), 201

@app.route('/projects/<int:project_id>/fund', methods=['POST'])
def fund_existing_project(project_id):
    data = request.json
    fund_project(project_id, data['amount'])
    return jsonify({"status": "success"}), 200
