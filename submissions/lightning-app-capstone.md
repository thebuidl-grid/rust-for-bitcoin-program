# SatsForGood – Lightning Donation Platform

This project is a simple Bitcoin Lightning donation platform that allows users to send donations using LNBits. It also includes a real-time dashboard for tracking paid donations.

The goal of this project is to demonstrate how a real-world fintech donation system can be built using a full-stack approach.

---

##  Live Links

- **Frontend (Vercel):**  
   [Visit Live Site](https://frontend-1jywat4i5-christopherdominics-projects.vercel.app)	

- **Backend API (Render):**  
  [Visit API](https://satsforgood-render.onrender.com)
---

##  Repositories

- **Frontend Repository:**  
  [GitHub Repo](https://github.com/Christopherdominic/SatsForGood.git)

- **Backend Repository:**  
  [Github Repo](https://github.com/Christopherdominic/SatsForGood-render.git)

## Lightning Node provider

  [Lnbits](https://lnbits.com)
	
---

## Features

- Create Lightning invoices using LNBits
- Track payment status (Pending / Paid)
- Live dashboard for recent donations
- Total donations and donor count
- Real-time updates from the backend API
- Fully deployed frontend and backend

---

##  Tech Stack

### Frontend
- Next.js
- TypeScript
- Tailwind CSS
- ShadCN UI

### Backend
- Django
- Django REST Framework
- LNBits API
- SQLite (for development/demo)
- Gunicorn (production server)

### Deployment
- Frontend hosted on **Vercel**
- Backend hosted on **Render**

---

##  How It Works (Flow)

1. A donor enters an amount and name on the frontend.
2. The frontend sends a request to the Django backend.
3. The backend creates a Lightning invoice using LNBits.
4. The donor pays the invoice.
5. The backend checks the payment status.
6. Once confirmed, the donation appears as **PAID** on the dashboard.

---

##  Project Status

- Backend:  Complete and deployed
- Frontend:  Complete and deployed
- API Integration:  Working
- Real Payments:  LNBits integration active

---

## Author

**Christopher Dominic Eze**  
Full Stack Developer (Web & Blockchain)


