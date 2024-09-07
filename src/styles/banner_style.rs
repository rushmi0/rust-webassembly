pub const STYLE: &str = r#"
@keyframes slideIn {
    from {
        transform: translateY(-30px);
        opacity: 0;
        filter: blur(10px);
    }
    to {
        transform: translateY(0);
        opacity: 1;
        filter: blur(0px);
    }
}

.title,
.topic,
.des,
.seeMore {
    opacity: 0;
    animation: slideIn 0.5s ease-in-out forwards;
}

.title {
    animation-delay: 1s;
}

.topic {
    animation-delay: 1.2s;
}

.des {
    animation-delay: 1.4s;
}

.seeMore {
    animation-delay: 1.6s;
}

"#;
