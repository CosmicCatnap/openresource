# This md sheet acts as the repos project tracker for items related to openresource and is my entire issue/ticket management system
# Modify this document whenever you have finished a feature or whenever you want to capture a deliverable suggestion, goal, or even just thought

# BACKEND TASKS
* Generate basic normalized schema with diesel
* Create basic CRUD route boilerplate for that schema

# FRONTEND TASKS
* Make a page for each "domain" that is reflected in the schema such as scheduling, inventory, projects, employees, etc
* Make a login page
* make a seperate landing page in some nonsense, doesn't really matter if its next or wordpress or whatever its just fluff

# INFRA
* Finish dockerizing 
* Create github runner script
* Setup deployment user and nginx on prod box

##LOG

* copied https://github.com/vercel/nextjs-postgres-nextauth-tailwindcss-template to frontend as a good starting point for a dashboard but need to uncouple its login mechanism from sso
