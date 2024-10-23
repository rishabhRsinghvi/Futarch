@app.route('/projects/<int:project_id>/refund', methods=['POST'])
def process_refund(project_id):
    process_refund(project_id)
    return jsonify({"status": "refund initiated"}), 200
