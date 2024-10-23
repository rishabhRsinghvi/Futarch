from flask import Flask
from controllers.projects_controller import app as projects_app
from controllers.milestones_controller import app as milestones_app
from controllers.refunds_controller import app as refunds_app

app = Flask(__name__)
app.register_blueprint(projects_app)
app.register_blueprint(milestones_app)
app.register_blueprint(refunds_app)

if __name__ == '__main__':
    app.run(debug=True)
