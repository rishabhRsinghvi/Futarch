from flask import Flask
from api.controllers.projects_controller import app as projects_app
from api.controllers.milestones_controller import app as milestones_app
from api.controllers.refunds_controller import app as refunds_app

app = Flask(__name__)

# Register the blueprints
app.register_blueprint(projects_app)
app.register_blueprint(milestones_app)
app.register_blueprint(refunds_app)

if __name__ == '__main__':
    app.run(debug=True)
