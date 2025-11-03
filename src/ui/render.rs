use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Clear},
    Frame,
};
use crate::app::state::{AppState, AppMode};
use crate::ui::modal::InputField;

pub fn render(frame: &mut Frame, state: &AppState) {
    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(frame.area());

    // Render list on the left
    render_list(frame, state, chunks[0]);

    // Render details on the right
    render_details(frame, state, chunks[1]);

    // Render any active modal or error on top
    if let Some(err) = &state.error_message {
        render_error_popup(frame, err);
    } else if state.mode == AppMode::ConfirmDelete {
        render_delete_confirm_modal(frame);
    } else if state.mode == AppMode::Filter {
        render_filter_input(frame, state);
    } else if state.mode == AppMode::AddFruit || state.mode == AppMode::EditFruit {
        if let Some(modal) = &state.modal {
            let title = if state.mode == AppMode::AddFruit {
                "Add Fruit"
            } else {
                "Edit Fruit"
            };
            render_fruit_modal(frame, modal, title);
        }
    }
}

fn render_list(frame: &mut Frame, state: &AppState, area: Rect) {
    let display_fruits = state.display_fruits();
    let items: Vec<ListItem> = display_fruits
        .iter()
        .map(|f| ListItem::new(f.name.as_str()))
        .collect();

    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(Some(state.selected_index));

    let title = if state.is_filtering() {
        format!(
            "Fruits — (filtered: {}/{}) [/] search, [a] add, [e] edit, [d] delete, [Esc] clear",
            display_fruits.len(),
            state.fruits.len()
        )
    } else {
        "Fruits — [↑/↓/j/k] navigate, [a] add, [e] edit, [d] delete, [/] search, [?] help"
            .to_string()
    };

    let list = List::new(items)
        .block(Block::default().title(title).borders(Borders::ALL))
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, area, &mut list_state);
}

fn render_details(frame: &mut Frame, state: &AppState, area: Rect) {
    let details = if state.fruits.is_empty() {
        Paragraph::new("No fruits available")
            .block(Block::default().title("Details").borders(Borders::ALL))
    } else if let Some(fruit) = state.selected_fruit() {
        let txt = format!(
            "Name: {}\n\nDimensions:\n  Length: {}\n  Width : {}\n  Height: {}\n\nVolume: {:.2}",
            fruit.name, fruit.length, fruit.width, fruit.height, fruit.volume()
        );
        Paragraph::new(txt).block(
            Block::default()
                .title(format!("Details [{}]", state.selected_index + 1))
                .borders(Borders::ALL),
        )
    } else {
        Paragraph::new("Select a fruit")
            .block(Block::default().title("Details").borders(Borders::ALL))
    };

    frame.render_widget(details, area);
}

fn render_filter_input(frame: &mut Frame, state: &AppState) {
    let popup_area = centered_rect(60, 10, frame.area());

    frame.render_widget(Clear, popup_area);
    let text = format!("> {}", state.filter_query);
    let para = Paragraph::new(text)
        .block(Block::default().title("Search").borders(Borders::ALL))
        .alignment(Alignment::Left);

    frame.render_widget(para, popup_area);
}

fn render_delete_confirm_modal(frame: &mut Frame) {
    let popup_area = centered_rect(50, 15, frame.area());

    frame.render_widget(Clear, popup_area);

    let lines = vec![
        Line::from("Are you sure you want to delete this fruit?"),
        Line::from(""),
        Line::from(vec![
            Span::raw("["),
            Span::styled("Y", Style::default().fg(Color::Yellow)),
            Span::raw("]es  ["),
            Span::styled("N", Style::default().fg(Color::Yellow)),
            Span::raw("]o"),
        ]),
    ];

    let para = Paragraph::new(lines)
        .block(Block::default().title("Confirm Delete").borders(Borders::ALL))
        .alignment(Alignment::Center);

    frame.render_widget(para, popup_area);
}

fn render_fruit_modal(frame: &mut Frame, modal: &crate::ui::modal::ModalState, title: &str) {
    let popup_area = centered_rect(60, 50, frame.area());
    frame.render_widget(Clear, popup_area);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(popup_area);

    // Name field
    let name_style = if modal.focused_field == InputField::Name {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let name_widget = Paragraph::new(modal.name.as_str())
        .block(Block::default().title("Name").borders(Borders::ALL))
        .style(name_style);
    frame.render_widget(name_widget, inner[0]);

    // Length field
    let length_style = if modal.focused_field == InputField::Length {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let length_widget = Paragraph::new(modal.length.as_str())
        .block(Block::default().title("Length").borders(Borders::ALL))
        .style(length_style);
    frame.render_widget(length_widget, inner[1]);

    // Width field
    let width_style = if modal.focused_field == InputField::Width {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let width_widget = Paragraph::new(modal.width.as_str())
        .block(Block::default().title("Width").borders(Borders::ALL))
        .style(width_style);
    frame.render_widget(width_widget, inner[2]);

    // Height field
    let height_style = if modal.focused_field == InputField::Height {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let height_widget = Paragraph::new(modal.height.as_str())
        .block(Block::default().title("Height").borders(Borders::ALL))
        .style(height_style);
    frame.render_widget(height_widget, inner[3]);

    // Instructions
    let instructions = Line::from(vec![
        Span::raw("["),
        Span::styled("Tab", Style::default().fg(Color::Cyan)),
        Span::raw("] next  ["),
        Span::styled("S-Tab", Style::default().fg(Color::Cyan)),
        Span::raw("] prev  ["),
        Span::styled("Enter", Style::default().fg(Color::Green)),
        Span::raw("] save  ["),
        Span::styled("Esc", Style::default().fg(Color::Red)),
        Span::raw("] cancel"),
    ]);
    let instructions_widget = Paragraph::new(instructions)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    frame.render_widget(instructions_widget, inner[4]);

    // Error message if present
    if let Some(err) = &modal.error {
        let error_area = centered_rect(50, 15, frame.area());
        frame.render_widget(
            Paragraph::new(err.as_str())
                .block(Block::default().title("Error").borders(Borders::ALL))
                .style(Style::default().fg(Color::Red))
                .alignment(Alignment::Center),
            error_area,
        );
    }

    // Border and title for the modal
    let border = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded);
    frame.render_widget(border, popup_area);
}

fn render_error_popup(frame: &mut Frame, message: &str) {
    let popup_area = centered_rect(70, 20, frame.area());

    frame.render_widget(Clear, popup_area);

    let para = Paragraph::new(message)
        .block(Block::default().title("Error").borders(Borders::ALL))
        .alignment(Alignment::Center);

    frame.render_widget(para, popup_area);
}

/// Helper function to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
